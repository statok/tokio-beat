use chrono::{DateTime, Utc};

pub struct Oneshot {
    at: DateTime<Utc>,
}

impl Oneshot {
    pub fn new(at: DateTime<Utc>) -> Oneshot {
        Oneshot { at }
    }

    pub fn next(&self) -> Option<DateTime<Utc>> {
        if Utc::now() <= self.at {
            Some(self.at)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::Duration;

    use super::*;

    #[test]
    fn test_next() {
        let timer = Oneshot::new(Utc::now() + Duration::seconds(1));
        assert!(timer.next().is_some());

        let timer = Oneshot::new(Utc::now() - Duration::seconds(1));
        assert!(timer.next().is_none());
    }
}
