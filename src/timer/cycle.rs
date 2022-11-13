use chrono::offset::TimeZone;
use chrono::{DateTime, Duration, Utc};

pub struct Cycle {
    start_ts: i64,
    interval: i64,
    end_ts: Option<i64>,
}

impl Cycle {
    pub fn new(
        start_at: DateTime<Utc>,
        end_at: Option<DateTime<Utc>>,
        interval: Duration,
    ) -> Cycle {
        Cycle {
            start_ts: start_at.timestamp_millis(),
            interval: interval.num_milliseconds(),
            end_ts: end_at.map(|e| e.timestamp_millis()),
        }
    }

    pub fn next(&self) -> Option<DateTime<Utc>> {
        let now_ts = Utc::now().timestamp_millis();
        if let Some(end_ts) = self.end_ts {
            if now_ts > end_ts {
                return None;
            }
        }

        let duration = now_ts - self.start_ts;
        let factor = duration / self.interval + 1;
        let next_ts = self.start_ts + factor * self.interval;

        Some(Utc.timestamp_millis(next_ts))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use chrono::{Duration, Utc};

    #[test]
    fn test_next() {
        let timer = Cycle::new(Utc::now(), None, Duration::seconds(3));
        assert!(timer.next().is_some());

        let timer = Cycle::new(
            Utc::now() - Duration::seconds(4),
            Some(Utc::now() - Duration::milliseconds(1)),
            Duration::seconds(3),
        );
        assert!(timer.next().is_none());
    }
}
