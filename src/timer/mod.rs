mod cron;
pub mod cycle;
mod oneshot;
use chrono::{DateTime, Utc};

pub(crate) use self::cron::Cron;
pub(crate) use self::cycle::Cycle;
pub(crate) use self::oneshot::Oneshot;

pub enum Timer {
    Cron(Cron),
    Oneshot(Oneshot),
    Cycle(Cycle),
}

impl Timer {
    pub fn next(&self) -> Option<DateTime<Utc>> {
        use Timer::*;
        match self {
            Cron(cron) => cron.next(),
            Oneshot(oneshot) => oneshot.next(),
            Cycle(cycle) => cycle.next(),
        }
    }
}
