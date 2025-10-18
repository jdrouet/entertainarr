#[derive(Clone, Copy, Debug)]
pub struct Page {
    pub limit: u32,
    pub offset: u32,
}

#[derive(Clone, Copy, Debug)]
pub enum SortOrder {
    Asc,
    Desc,
}

#[derive(Clone, Copy, Debug)]
pub struct Sort<Field> {
    pub field: Field,
    pub order: SortOrder,
}
