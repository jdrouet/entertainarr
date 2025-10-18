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
}
