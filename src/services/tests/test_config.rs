use serde_json::json;

use crate::services::resource_manager::ResourceManagerConfig;

#[test]
fn test_resource_manager_config_fields() {
    let config: ResourceManagerConfig = serde_json::from_value(json!({
        "base_url": "http://localhost:8082",
        "timeout": 5,
    }))
    .expect("failed to deserialize config");

    assert_eq!("http://localhost:8082", config.base_url());
    assert_eq!(5, config.timeout());
}

#[test]
fn test_resource_manager_config_requires_all_fields() {
    let result: Result<ResourceManagerConfig, _> =
        serde_json::from_value(json!({ "base_url": "http://localhost:8082" }));

    assert!(result.is_err());
}
