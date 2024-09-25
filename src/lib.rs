pub mod pages;
pub mod components;
pub mod router;

pub mod app_ctx {
    #[derive(Debug, Clone, PartialEq)]
    pub struct AppContext {
        pub document_title: String,
    }
}
