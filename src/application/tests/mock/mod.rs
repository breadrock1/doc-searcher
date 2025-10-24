pub mod cacher;
pub mod storage;
pub mod um;

use rstest::fixture;

pub struct TestEnvironment {
    pub cacher: cacher::MockCacher,
    pub storage: storage::MockStorage,
    pub um: um::MockUserManagerClient,
}

#[fixture]
pub fn init_test_environment() -> TestEnvironment {
    let cacher = cacher::MockCacher::new();
    let storage = storage::MockStorage::new();
    let um = um::MockUserManagerClient::new();
    TestEnvironment {
        cacher,
        storage,
        um,
    }
}
