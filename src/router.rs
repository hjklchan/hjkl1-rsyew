use crate::pages::{about::About, home::Home, not_found::NotFound, posts::Posts};
use strum_macros::EnumIter;
use yew::{html, Html};
use yew_router::Routable;

#[derive(Clone, Routable, PartialEq, EnumIter)]
pub enum Router {
    #[at("/")]
    Home,
    #[at("/posts")]
    Posts,
    #[at("/posts/:id")]
    PostDetail { id: u64 },
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
        Router::PostDetail { id } => html!(<>{"Post Detail Page, #"}{id}</>),
        Router::About => html!(<About />),
        Router::NotFound => html!(<NotFound />),
    }
}
