use yew::{function_component, html, Html};

#[function_component(NotFound)]
pub fn not_found() -> Html {
    html! {
        "Page Not Found"
    }
}
