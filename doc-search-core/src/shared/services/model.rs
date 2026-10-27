/// Information about a resource location.
///
/// Describes the search index and prefix where a resource's data is stored.
#[derive(Clone, Debug)]
pub struct ResourceInfo {
    pub index: String,
    pub prefix: String,
}
