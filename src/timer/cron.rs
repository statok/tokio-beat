use chrono::{DateTime, FixedOffset, Utc};
use cron::Schedule;

pub struct Cron {
    schedule: Schedule,
    tz: FixedOffset,
}

impl Cron {
    pub fn new(schedule: Schedule, time_zone: FixedOffset) -> Cron {
        Cron {
            schedule,
            tz: time_zone,
        }
    }

    pub fn next(&self) -> Option<DateTime<Utc>> {
        if let Some(when) = self.schedule.upcoming(self.tz).next() {
            Some(when.into())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::Offset;

    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_next() {
        let timer = Cron::new(Schedule::from_str("0 */1 * * * *").unwrap(), Utc.fix());

        let next = timer.next();
        assert!(next.is_some());
    }
}
