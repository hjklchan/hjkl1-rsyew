use crate::app_ctx::AppContext;
use crate::components::layouts::{Footer, Header};
use crate::router::{switch, Router};
use gloo::console::log;
use gloo_net::http::Request;
use yew::{
    function_component, html, use_effect_with, use_state, ContextProvider, Html, UseStateHandle,
};
use yew_router::{BrowserRouter, Switch};

#[function_component(App)]
pub fn app() -> Html {
    let app_ctx: UseStateHandle<AppContext> = use_state(|| AppContext {
        document_title: "Hjkl1 App".to_string(),
        // None         =>  Checking
        // Some(true)   =>  Available
        // Some(false)  =>  Unavailable
        server_status: None, // @deprecated
    });

    // Check server status
    //
    // Listen api called `SERVER_ADDRESS/health`
    {
        let cloned_app_ctx = app_ctx.clone();

        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let response = Request::get("http://127.0.0.1:8000/health").send().await;
                let mut tmp_app_ctx = (*cloned_app_ctx).clone();

                match response {
                    Ok(response) if response.status() == 200 => {
                        tmp_app_ctx.server_status = Some(true);
                        cloned_app_ctx.set(tmp_app_ctx);
                        log!("server is ok");
                    },
                    Ok(_) => {
                        tmp_app_ctx.server_status = Some(false);
                        cloned_app_ctx.set(tmp_app_ctx);
                        log!("server is unavailable1");
                    },
                    Err(_) => {
                        tmp_app_ctx.server_status = Some(false);
                        cloned_app_ctx.set(tmp_app_ctx);
                        log!("server is unavailable2");
                    },
                }
            });
        })
    }

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
