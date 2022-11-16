use chrono::{DateTime, Utc};
use uuid::Uuid;

pub struct Context {
    pub job_id: Uuid,
    pub when: DateTime<Utc>,
}
