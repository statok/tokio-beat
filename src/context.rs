use crate::scheduler::Scheduler;
use chrono::{DateTime, Utc};
use std::sync::Arc;
use uuid::Uuid;

pub struct Context {
    pub job_id: Uuid,
    pub when: DateTime<Utc>,
}
