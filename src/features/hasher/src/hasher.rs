#[derive(Default)]
pub struct Hashed {
    hash_data: String,
}

impl Hashed {
    pub fn new(hashed: String) -> Hashed {
        Hashed {
            hash_data: hashed,
        }
    }

    pub fn get_hash_data(&self) -> &str {
        self.hash_data.as_str()
    }
}
