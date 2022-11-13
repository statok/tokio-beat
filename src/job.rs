use crate::time_event::TimeEvent;
use crate::timer;
use crate::timer::Timer;
use chrono::{DateTime, Duration, FixedOffset, Utc};
use futures::future::BoxFuture;
use std::sync::Arc;
use uuid::Uuid;

pub struct Job {
    id: Uuid,
    timer: Timer,
    f: Arc<dyn Fn(Uuid) -> BoxFuture<'static, ()> + Send + Sync>,
}

impl Job {
    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn f(&self) -> Arc<dyn Fn(Uuid) -> BoxFuture<'static, ()> + Send + Sync> {
        self.f.clone()
    }

    pub fn next_time_event(&self) -> Option<TimeEvent> {
        self.timer.next().map(|when| TimeEvent {
            when,
            job_id: self.id,
        })
    }

    pub fn cron(expr: &str) -> CronJobBuilder {
        CronJobBuilder::new(expr)
    }

    pub fn cycle<T: Into<DateTime<Utc>>>(start_at: T, interval: Duration) -> CycleJobBuilder {
        CycleJobBuilder::new(start_at, interval)
    }

    pub fn oneshot<T: Into<DateTime<Utc>>>(at: T) -> OneshotJobBuilder {
        OneshotJobBuilder::new(at)
    }
}

pub struct CronJobBuilder {
    expr: String,
    tz: FixedOffset,
}

impl CronJobBuilder {
    fn new(expr: &str) -> CronJobBuilder {
        CronJobBuilder {
            expr: expr.into(),
            tz: FixedOffset::east(0),
        }
    }

    pub fn tz<Z: Into<FixedOffset>>(mut self, time_zone: Z) -> CronJobBuilder {
        self.tz = time_zone.into();
        self
    }

    pub fn do_<T>(&mut self, f: T) -> Job
    where
        T: Fn(Uuid) -> BoxFuture<'static, ()> + 'static + Send + Sync,
    {
        use cron::Schedule;
        use std::str::FromStr;

        let shed = Schedule::from_str(&self.expr).unwrap();
        Job {
            id: Uuid::new_v4(),
            f: Arc::new(f),
            timer: Timer::Cron(timer::Cron::new(shed, self.tz)),
        }
    }
}

pub struct CycleJobBuilder {
    start_at: DateTime<Utc>,
    end_at: Option<DateTime<Utc>>,
    interval: Duration,
}

impl CycleJobBuilder {
    fn new<T: Into<DateTime<Utc>>>(start_at: T, interval: Duration) -> CycleJobBuilder {
        CycleJobBuilder {
            start_at: start_at.into(),
            end_at: None,
            interval,
        }
    }

    pub fn end_at(mut self, end_at: &DateTime<FixedOffset>) -> CycleJobBuilder {
        self.end_at = Some(end_at.to_owned().into());
        self
    }

    pub fn do_<T>(&mut self, f: T) -> Job
    where
        T: Fn(Uuid) -> BoxFuture<'static, ()> + 'static + Send + Sync,
    {
        Job {
            id: Uuid::new_v4(),
            f: Arc::new(f),
            timer: Timer::Cycle(timer::cycle::Cycle::new(
                self.start_at,
                self.end_at,
                self.interval,
            )),
        }
    }
}

pub struct OneshotJobBuilder {
    at: DateTime<Utc>,
}

impl OneshotJobBuilder {
    pub fn new<T: Into<DateTime<Utc>>>(at: T) -> OneshotJobBuilder {
        OneshotJobBuilder { at: at.into() }
    }

    pub fn do_<T>(&mut self, f: T) -> Job
    where
        T: Fn(Uuid) -> BoxFuture<'static, ()> + 'static + Send + Sync,
    {
        Job {
            id: Uuid::new_v4(),
            f: Arc::new(f),
            timer: Timer::Oneshot(timer::Oneshot::new(self.at)),
        }
    }
}
