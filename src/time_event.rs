use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct TimeEvent {
    pub job_id: Uuid,
    pub when: DateTime<Utc>,
}

impl Ord for TimeEvent {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.when.cmp(&other.when)
    }
}

impl PartialOrd for TimeEvent {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for TimeEvent {
    fn eq(&self, other: &Self) -> bool {
        self.when == other.when
    }
}

impl Eq for TimeEvent {}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::prelude::*;

    #[test]
    fn test_ord() {
        let t1 = Utc.ymd(2022, 10, 31).and_hms(22, 0, 0);
        let t2 = Utc.ymd(2022, 10, 31).and_hms(22, 1, 0);
        let t3 = Utc.ymd(2022, 10, 31).and_hms(22, 1, 0);
        let t4 = Utc.ymd(2022, 10, 31).and_hms(23, 1, 0);

        let e1 = TimeEvent {
            job_id: Uuid::new_v4(),
            when: t1,
        };
        let e2 = TimeEvent {
            job_id: Uuid::new_v4(),
            when: t2,
        };
        let e3 = TimeEvent {
            job_id: Uuid::new_v4(),
            when: t3,
        };
        let e4 = TimeEvent {
            job_id: Uuid::new_v4(),
            when: t4,
        };

        assert!(e1 < e2);
        assert!(e1 <= e2);
        assert!(e2 > e1);
        assert!(e2 >= e1);

        assert!(e2 == e3);
        assert!(e2 >= e3);
        assert!(e2 <= e3);

        assert!(e3 < e4);
        assert!(e3 <= e4);
        assert!(e4 > e3);
        assert!(e4 >= e3);
    }
}
