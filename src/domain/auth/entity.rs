#[nutype::nutype(
    sanitize(trim, lowercase),
    validate(not_empty),
    derive(Debug, PartialEq)
)]
pub struct Email(String);

#[nutype::nutype(
    sanitize(trim),
    validate(not_empty, len_char_min = 8),
    derive(Debug, PartialEq)
)]
pub struct Password(String);

#[derive(Clone, Debug, PartialEq)]
pub struct Profile {
    pub id: u64,
    pub password: String,
}

impl Profile {
    pub(super) fn compare_password(&self, clear: &str) -> bool {
        use base64ct::Encoding;
        use sha2::Digest;

        let mut hasher = sha2::Sha256::new();
        hasher.update(&self.id.to_be_bytes());
        hasher.update(clear.as_bytes());
        let hash = hasher.finalize();
        base64ct::Base64::encode_string(&hash) == self.password
    }
}
