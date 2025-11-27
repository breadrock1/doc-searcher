pub mod storage;

use rstest::fixture;

pub struct TestEnvironment {
    pub storage: storage::MockStorage,
}

#[fixture]
pub fn init_test_environment() -> TestEnvironment {
    let storage = storage::MockStorage::new();
    TestEnvironment { storage }
}
