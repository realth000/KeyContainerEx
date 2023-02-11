use crate::storage::uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct DeletedObject {
    pub uuid: Uuid,
    pub deletion_time: DateTime<Utc>,
}

impl DeletedObject {
    pub fn new(uuid: Uuid, datetime: DateTime<Utc>) -> Self {
        DeletedObject {
            uuid,
            deletion_time: datetime,
        }
    }
}
