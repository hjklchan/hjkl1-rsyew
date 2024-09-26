use crate::pages::{
    about::About,
    auth::{CreateAccount, Login},
    home::Home,
    not_found::NotFound,
    posts::{PostDetail, Posts, CreatePost},
};
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
    #[at("/posts/create")]
    CreatePost,
    #[at("/about")]
    About,
    #[at("/auth/login")]
    Login,
    #[at("/auth/create_account")]
    CreateAccount,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Router) -> Html {
    match routes {
        Router::Home => html!(<Home />),
        Router::Posts => html!(<Posts />),
        Router::PostDetail { id } => html!(<PostDetail id={id} />),
        Router::CreatePost => html!(<CreatePost />),
        Router::About => html!(<About />),
        Router::Login => html!(<Login />),
        Router::CreateAccount => html!(<CreateAccount />),
        Router::NotFound => html!(<NotFound />),
    }
}
