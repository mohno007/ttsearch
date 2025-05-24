use serde::Serialize;
use chrono::{DateTime, TimeZone, Utc};

#[derive(Debug, Serialize)]
pub struct Message {
    topic_id: u64,
    id: u64,
    account_name: String,
    account_fullname: String,
    message: String,
    created_at: DateTime<Utc>,
}

impl Message {
    pub fn new<Tz: TimeZone>(
        topic_id: u64,
        id: u64,
        account_name: &str,
        account_fullname: &str,
        message: &str,
        created_at: DateTime<Tz>,
    ) -> Self {
        Self {
            topic_id,
            id,
            account_name: account_name.to_owned(),
            account_fullname: account_fullname.to_owned(),
            message: message.to_owned(),
            created_at: created_at.to_utc()
        }
    }

    pub fn topic_id(&self) -> u64 {
        self.topic_id
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn account_name(&self) -> &str {
        &self.account_name
    }

    pub fn account_fullname(&self) -> &str {
        &self.account_fullname
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn created_at(&self) -> &DateTime<Utc> {
        &self.created_at
    }
}
