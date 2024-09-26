use crate::router::Router;
use yew::{function_component, html, Callback, Html};
use yew_router::{
    hooks::{use_location, use_navigator},
    prelude::Link,
    Routable,
};

#[function_component(Header)]
pub fn header() -> Html {
    let location = use_location().expect("location not available");
    let navigator = use_navigator().unwrap();

    let highlight_color = |path: &str| {
        if location.path().contains(path) {
            "text-[#369] font-semibold"
        } else {
            "text-gray-800"
        }
    };

    let on_login_click: Callback<(), ()> = {
        let cloned_navigator = navigator.clone();

        Callback::from(move |_| {
            cloned_navigator.push(&Router::Login);
        })
    };

    let on_create_account_click: Callback<(), ()> = {
        let cloned_navigator = navigator.clone();

        Callback::from(move |_| {
            cloned_navigator.push(&Router::CreateAccount);
        })
    };

    html! {
        <header class={"font-serif z-50 bg-gray-100 sticky top-0 w-full border-b-1 shadow-md bg-gradient-to-b from-gray-50 to-gray-300"}>
            <div class="flex justify-between px-6 items-center">
                <div>
                    <nav class="mx-auto py-1">
                        <div class="flex items-center">
                            <Link<Router> to={Router::Home} classes={"select-none font-semibold text-sm"}>
                                <span class={"px-2 bg-[#369] rounded-full shadow-inner text-neutral-100 hover:bg-blue-700"}>
                                    {"hjkl1"}<span class="text-red-500">{".rs"}</span>
                                </span>
                            </Link<Router>>
                            <div class="flex space-x-3 ml-3 text-xs items-center">
                                <Link<Router>
                                    to={Router::Posts}
                                    classes={highlight_color(&Router::Posts.to_path())}
                                >
                                    {"Posts"}
                                </Link<Router>>
                                <Link<Router>
                                    to={Router::About}
                                    classes={highlight_color(&Router::About.to_path())}
                                >
                                    {"About"}
                                </Link<Router>>
                            </div>
                        </div>
                    </nav>
                </div>
                <div class="space-x-1">
                    <button
                        class="text-xs"
                        onclick={
                            move |_| {
                                on_login_click.emit(());
                            }
                        }
                    >{"Login"}</button>
                    {"|"}
                    <button
                        onclick={
                            move |_| {
                                on_create_account_click.emit(());
                            }
                        }
                        class="text-xs"
                    >{"Create Account"}</button>
                </div>
            </div>
        </header>
    }
}
