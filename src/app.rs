use crate::app_ctx::AppContext;
use crate::router::{switch, Router};
use yew::{
    function_component, html, use_effect_with, use_state, ContextProvider, Html, UseStateHandle,
};
use yew_router::{BrowserRouter, Switch};
use crate::components::layouts::{Header, Footer};

#[function_component(App)]
pub fn app() -> Html {
    let app_ctx: UseStateHandle<AppContext> = use_state(|| AppContext {
        document_title: "Hjkl1 App".to_string(),
    });

    // Show app name in document title
    {
        let cloned_app_ctx = app_ctx.clone();

        use_effect_with((), move |_| {
            web_sys::console::log_1(&"app is rendered".into());
            web_sys::window()
                .unwrap()
                .document()
                .unwrap()
                .set_title(&*cloned_app_ctx.document_title);
        })
    }

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
