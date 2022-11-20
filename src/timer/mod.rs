mod cron;
pub mod cycle;
mod oneshot;
use chrono::{DateTime, Utc};

pub(crate) use self::cron::Cron;
pub(crate) use self::cycle::Cycle;
pub(crate) use self::oneshot::Oneshot;

/// An enum representing the various forms of a timer.
pub enum Timer {
    /// A crontab timer
    ///
    /// This timer based on a cron schedule.
    Cron(Cron),
    /// A oneshot timer
    ///
    /// This timer only produce a certain datetime in the future.
    /// After that datetime, it produce nothing to indicate the schedule is done.
    Oneshot(Oneshot),
    /// A cycle timer
    ///
    /// This timer produce incoming datetime based on a start datetime and a fixed interval.
    Cycle(Cycle),
}

impl Timer {
    /// Return the nearest incoming datetime from now for the timer.
    /// None indicates that the schedule on which this timer based is done.
    pub fn next(&self) -> Option<DateTime<Utc>> {
        use Timer::*;
        match self {
            Cron(cron) => cron.next(),
            Oneshot(oneshot) => oneshot.next(),
            Cycle(cycle) => cycle.next(),
        }
    }
}
