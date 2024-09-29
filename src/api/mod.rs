use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct OhMyResponse<T> {
    pub message: String,
    pub data: T,
}

#[derive(Debug, Deserialize)]
pub struct PaginationData<T> {
    pub items: Vec<T>,
    pub page_size: u64,
    pub has_prev: bool,
    pub has_next: bool,
    pub total: u64,
}