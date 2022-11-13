use crate::job::Job;
use crate::time_event::TimeEvent;
use chrono::{DateTime, Utc};
use std::cmp::Reverse;
use std::{
    collections::{BinaryHeap, HashMap},
    sync::{Arc, Mutex},
};
use tokio::sync::Notify;
use tokio::task::JoinHandle;
use tokio::time;
use tracing::info;
use uuid::Uuid;

#[derive(Clone)]
pub struct Scheduler {
    shared: Arc<Shared>,
}

struct Shared {
    state: Mutex<State>,
    background_task: Notify,
}

struct State {
    job_store: HashMap<Uuid, Job>,
    new_jobs: Vec<Job>,
    time_events: BinaryHeap<Reverse<TimeEvent>>,
    shutdown: bool,
}

impl Shared {
    fn make_time_events(&self) {
        let mut state = self.state.lock().unwrap();
        let state = &mut *state;

        state.new_jobs.iter().for_each(|job| {
            state
                .time_events
                .push(Reverse(job.next_time_event().unwrap()));
        });

        state.new_jobs.drain(0..).for_each(|job| {
            state.job_store.insert(job.id(), job);
        });
    }

    fn process_time_events(&self) -> Option<DateTime<Utc>> {
        let mut state = self.state.lock().unwrap();

        if state.shutdown {
            return None;
        }

        let state = &mut *state;

        let now = Utc::now();
        while let Some(Reverse(TimeEvent { when, job_id })) = state.time_events.peek() {
            if when > &now {
                return Some(when.to_owned());
            }

            if let Some(job) = state.job_store.get_mut(job_id) {
                let f = job.f();
                let fut = f(job_id.to_owned());
                tokio::spawn(async move {
                    fut.await;
                });

                state.time_events.pop();
                if let Some(next_time_event) = job.next_time_event() {
                    state.time_events.push(Reverse(next_time_event));
                }
            }
        }
        None
    }

    fn is_shutdown(&self) -> bool {
        self.state.lock().unwrap().shutdown
    }
}

impl Scheduler {
    pub fn new() -> Scheduler {
        let shared = Arc::new(Shared {
            state: Mutex::new(State {
                job_store: HashMap::new(),
                new_jobs: Vec::new(),
                time_events: BinaryHeap::new(),
                shutdown: false,
            }),
            background_task: Notify::new(),
        });

        Scheduler { shared }
    }
}

impl Scheduler {
    pub fn add_job(&self, job: Job) {
        let mut state = self.shared.state.lock().unwrap();
        state.new_jobs.push(job);
        drop(state);

        self.shared.background_task.notify_one();
    }

    pub fn remove_job(&self, job_id: &Uuid) {
        let mut state = self.shared.state.lock().unwrap();
        state.job_store.remove(job_id);
        drop(state);

        self.shared.background_task.notify_one();
    }

    pub fn start(&mut self) -> JoinHandle<()> {
        tokio::spawn(schedule_jobs(self.shared.clone()))
    }

    pub fn shutdown(&self) {
        let mut state = self.shared.state.lock().unwrap();
        state.shutdown = true;
        drop(state);

        self.shared.background_task.notify_one();
    }
}

async fn schedule_jobs(shared: Arc<Shared>) {
    while !shared.is_shutdown() {
        shared.make_time_events();

        let when = shared.process_time_events();
        if let Some(when) = when {
            let remaining = when - Utc::now();
            tokio::select! {
                _ = time::sleep(remaining.to_std().unwrap_or_else(|_| std::time::Duration::from_secs(0))) => {}
                _ = shared.background_task.notified() => {}
            }
        } else {
            shared.background_task.notified().await;
        }
    }

    info!("Schedule jobs background task shut down")
}
