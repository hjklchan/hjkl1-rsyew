use gloo_net::http::{Request, RequestBuilder};

pub async fn fetch_with_interceptor(url: &str) -> Request {
    RequestBuilder::new(url).build().unwrap()
}
