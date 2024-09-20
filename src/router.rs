use crate::pages::{about::About, home::Home, not_found::{self, NotFound}, posts::Posts};
use yew::{html, Html};
use yew_router::Routable;
use strum_macros::EnumIter;

#[derive(Clone, Routable, PartialEq, EnumIter)]
pub enum Router {
    #[at("/")]
    Home,
    #[at("/posts")]
    Posts,
    #[at("/posts/:id")]
    PostDetail {id: String},
    #[at("/about")]
    About,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Router) -> Html {
    match routes {
        Router::Home => html!(<Home />),
        Router::Posts => html!(<Posts />),
        Router::PostDetail { id } => html!(<>{"Post Detail Page"}</>),
        Router::About => html!(<About />),
        Router::NotFound => html!(<NotFound />),
    }
}
