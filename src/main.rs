use hjkl1_yew::components::layouts::{Footer, Header};
use hjkl1_yew::router::{switch, Router};
use yew::prelude::*;
use yew_router::{BrowserRouter, Switch};

#[derive(Debug, Clone, PartialEq)]
pub struct AppContext {
    pub document_title: String,
}

#[function_component(App)]
fn app() -> Html {
    let app_ctx: UseStateHandle<AppContext> = use_state(|| AppContext {
        document_title: "hjkl1 app".to_string()
    });

    html! {
        <>
            // Global context data
            <ContextProvider<AppContext> context={(*app_ctx).clone()}>
                <BrowserRouter>
                    <Header />
                    <main class="overflow-y-auto p-6 font-serif">
                        <Switch<Router> render={switch} />
                    </main>
                    <Footer />
                </BrowserRouter>
            </ContextProvider<AppContext>>
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
