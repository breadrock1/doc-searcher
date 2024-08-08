extern crate hasher;

#[cfg(test)]
mod tests {
    use hasher::{gen_hash, HashType};

    const TEST_STRING: &str = "There is some data to check ssdeep hasher";

    #[ignore]
    #[test]
    fn ssdeep_hash_test() {
        let hasher_result = gen_hash(HashType::SSDEEP, TEST_STRING.as_bytes());
        let binding = hasher_result.unwrap();
        let hash_data = binding.get_hash_data();
        assert_eq!(hash_data, "3:ZFkREaLGqnP3/SX:7knL7v/k")
    }

    #[test]
    fn md5_hash_test() {
        let hasher_result = gen_hash(HashType::MD5, TEST_STRING.as_bytes());
        let binding = hasher_result.unwrap();
        let hash_data = binding.get_hash_data();
        assert_eq!(hash_data, "ece0157cd8e0c1c4d7986904151e7930")
    }
}
