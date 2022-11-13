use chrono::{Duration, Utc};
use tokio;
use tokio_beat::job::Job;
use tokio_beat::scheduler::Scheduler;
use tracing::info;

struct Foo {
    bar: String,
}

impl Foo {
    pub fn new() -> Foo {
        Foo {
            bar: "Hello, world!".into(),
        }
    }

    pub fn set_bar(&mut self, value: &str) {
        self.bar = value.into();
    }
}

#[tokio::main]
async fn main() -> () {
    tracing_subscriber::fmt::try_init().ok();

    let mut scheduler = Scheduler::new();

    scheduler.add_job(Job::cron("*/30 * * * * *").do_(|| {
        let mut foo = Foo::new();
        Box::pin(async move {
            info!("I am a cron job run every 30 seconds.");
            foo.set_bar("value");
        })
    }));

    scheduler.add_job(Job::cron("0 * * * * *").do_(|| {
        let mut foo = Foo::new();
        Box::pin(async move {
            info!("I am a cron job run every 1 minutes.");
            foo.set_bar("value");
        })
    }));

    scheduler.add_job(Job::cycle(Utc::now(), Duration::seconds(10)).do_(|| {
        let mut foo = Foo::new();
        Box::pin(async move {
            info!("I am a cycle job run every 10 seconds.");
            foo.set_bar("value");
        })
    }));

    let now = Utc::now();
    let after_10s = now + Duration::seconds(10);
    let after_10s_clone = after_10s.clone();
    scheduler.add_job(Job::oneshot(after_10s).do_(move || {
        let mut foo = Foo::new();
        Box::pin(async move {
            info!("I an a oneshot job run at {}.", after_10s_clone);
            foo.set_bar("value");
        })
    }));

    tokio::select! {
        _ = scheduler.start() => {
        }
        _ = tokio::signal::ctrl_c() => {
            info!("shutting down");
            scheduler.shutdown();
        }
    }
}
