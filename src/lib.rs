pub mod app;
pub mod api;
pub mod pages;
pub mod components;
pub mod router;
pub mod utils;

pub mod app_ctx {
    #[derive(Debug, Clone, PartialEq)]
    pub struct AppContext {
        pub document_title: String,
        // None         =>  Checking
        // Some(true)   =>  Available
        // Some(false)  =>  Unavailable
        pub server_status: Option<bool>, // @deprecated
    }
}
