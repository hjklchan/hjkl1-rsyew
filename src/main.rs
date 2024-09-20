use hjkl1_yew::components::layouts::{Footer, Header};
use hjkl1_yew::router::{switch, Router};
use yew::prelude::*;
use yew_router::{BrowserRouter, Switch};

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <BrowserRouter>
                <Header />
                <main class="overflow-y-auto p-6 font-serif">
                    <Switch<Router> render={switch} />
                </main>
                <Footer />
            </BrowserRouter>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
