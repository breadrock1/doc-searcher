use crate::shared::services::resource_manager::{ResourceId, ResourceManagerError};

#[test]
fn resource_manager_errors_display() {
    let nf = ResourceManagerError::NotFound("53079efa".into());
    assert_eq!("resource not found: 53079efa", nf.to_string());
    let ie = ResourceManagerError::InternalError("boom".into());
    assert_eq!("resource manager error: boom", ie.to_string());
}

#[test]
fn resource_id_wraps_string() {
    let id = ResourceId::from("53079efa-b0f2-4058-96dc-c792531972db".to_string());
    assert_eq!(id.as_ref(), "53079efa-b0f2-4058-96dc-c792531972db");
    assert_eq!(id.to_string(), "53079efa-b0f2-4058-96dc-c792531972db");
}
