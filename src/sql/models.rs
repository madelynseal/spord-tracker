use chrono::{DateTime, Local, Utc};

#[derive(Debug, PartialEq, Clone)]
pub enum SpordState {
    Pending,
    Ordered,
    Received,
    Other,
}
impl SpordState {
    pub fn as_sql(&self) -> i32 {
        match self {
            Self::Pending => 1,
            Self::Ordered => 2,
            Self::Received => 3,
            Self::Other => 0,
        }
    }
    pub fn from_sql(num: i32) -> SpordState {
        match num {
            1 => Self::Pending,
            2 => Self::Ordered,
            3 => Self::Received,
            _ => Self::Other,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SpordRecord {
    pub id: i32,
    pub customer_name: String,
    pub customer_phone: Option<String>,
    pub customer_email: Option<String>,
    pub part: String,
    pub state: SpordState,
    pub creation_date: DateTime<Utc>,
    pub received_date: Option<DateTime<Utc>>,
    pub comments: Option<String>,
}
impl SpordRecord {
    pub fn received_date_unix(&self) -> Option<i64> {
        if let Some(received_date) = self.received_date {
            Some(received_date.timestamp())
        } else {
            None
        }
    }
}
