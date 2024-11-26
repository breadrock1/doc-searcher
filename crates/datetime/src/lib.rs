use chrono::{DateTime, ParseResult, TimeZone, Utc};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::time::SystemTime;

const DATE_TIME_FORMAT: &str = "%Y-%m-%dT%H:%M:%SZ";

pub fn get_local_now() -> SystemTime {
    SystemTime::now()
}

pub fn serialize_dt<S>(dt: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    if let Some(dt) = dt {
        dt.format(DATE_TIME_FORMAT)
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
        .map(|value| format_datetime(value.as_str()))
        .map(|value| value.ok())
}

fn format_datetime(value: &str) -> ParseResult<DateTime<Utc>> {
    #[allow(deprecated)]
    Utc.datetime_from_str(value, DATE_TIME_FORMAT)
}
