extern crate datetime;

#[cfg(test)]
mod test {
    use chrono::{DateTime, TimeZone, Utc};
    use datetime::{deserialize_dt, get_local_now, serialize_dt};
    use serde::Deserialize;
    use serde_json::{json, to_value};

    #[derive(serde_derive::Deserialize, serde_derive::Serialize)]
    struct SomeDatetime {
        #[serde(
            serialize_with = "serialize_dt",
            deserialize_with = "deserialize_dt",
            skip_serializing_if = "Option::is_none"
        )]
        pub datetime: Option<DateTime<Utc>>,
    }

    impl SomeDatetime {
        pub fn new(datetime_str: &str) -> Self {
            #[allow(deprecated)]
            let datetime = Utc
                .datetime_from_str(datetime_str, "%Y-%m-%dT%H:%M:%SZ")
                .unwrap();
            SomeDatetime {
                datetime: Some(datetime),
            }
        }
    }

    #[test]
    fn serialize_dt_test() {
        let some_datetime = SomeDatetime::new("2023-01-11T00:00:00Z");
        let value = to_value(&some_datetime).unwrap();
        let some_str = value[&"datetime"].as_str().unwrap();
        assert_eq!(some_str, "2023-01-11T00:00:00Z")
    }

    #[test]
    fn deserialize_dt_test() {
        let value = json!({ "datetime": "2023-01-11T00:00:00Z" });
        let some_datetime = SomeDatetime::deserialize(value).unwrap();
        let dt_string = some_datetime
            .datetime
            .unwrap()
            .format("%Y-%m-%dT%H:%M:%SZ")
            .to_string();

        assert_eq!(dt_string.as_str(), "2023-01-11T00:00:00Z");
    }

    #[test]
    fn get_local_now_test() {
        let _ = get_local_now();
    }
}
