use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Recurrence {
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Pending,
    Completed,
    Recurring,
    Deleted,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy, Eq)]
pub enum Priority {
    H,
    M,
    L,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Eq)]
pub struct ContextSwitchMetadata {
    pub test: u32,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Eq)]
pub struct TaskId(pub Uuid);

impl fmt::Display for TaskId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Task {
    pub id: TaskId,
    #[serde(with = "tw_date_format")]
    pub entry: DateTime<Utc>,
    #[serde(with = "tw_date_format")]
    pub modified: DateTime<Utc>,
    pub status: Status,
    pub description: String,
    pub urgency: f64,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "opt_tw_date_format"
    )]
    pub due: Option<DateTime<Utc>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "opt_tw_date_format"
    )]
    pub start: Option<DateTime<Utc>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "opt_tw_date_format"
    )]
    pub end: Option<DateTime<Utc>>,
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "opt_tw_date_format"
    )]
    pub wait: Option<DateTime<Utc>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<Uuid>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<Priority>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub recur: Option<Recurrence>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contextswitch: Option<ContextSwitchMetadata>,
}

#[derive(Deserialize, Serialize)]
pub struct NewTask {
    pub definition: String,
}

pub mod tw_date_format {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y%m%dT%H%M%SZ";

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = format!("{}", date.format(FORMAT));
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT)
            .map_err(serde::de::Error::custom)
    }
}

pub mod opt_tw_date_format {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y%m%dT%H%M%SZ";

    pub fn serialize<S>(date: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(ref d) = *date {
            return serializer.serialize_str(&d.format(FORMAT).to_string());
        }

        serializer.serialize_none()
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT)
            .map(Some)
            .map_err(serde::de::Error::custom)
    }
}
