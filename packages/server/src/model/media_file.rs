#[derive(Debug)]
pub struct Entity {
    pub id: usize,
    pub source: Box<str>,
    pub path: Box<str>,
    pub size: usize,
}
