use chrono::{DateTime, ParseResult, TimeZone, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::time::SystemTime;

pub fn get_local_now() -> SystemTime {
    SystemTime::now()
}

pub fn serialize_dt<S>(dt: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(dt) = dt {
        dt.format("%Y-%m-%dT%H:%M:%SZ")
            .to_string()
            .serialize(serializer)
    } else {
        serializer.serialize_none()
    }
}

pub fn deserialize_dt<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    String::deserialize(deserializer)
        .and_then(|value| Ok(format_datetime(value.as_str())))
        .and_then(|value| Ok(value.ok()))
}

fn format_datetime(value: &str) -> ParseResult<DateTime<Utc>> {
    #[allow(deprecated)]
    Utc.datetime_from_str(value, "%Y-%m-%dT%H:%M:%SZ")
}
