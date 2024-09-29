use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use web_sys::{wasm_bindgen::JsCast, HtmlInputElement};
use yew::prelude::*;

use crate::api::OhMyResponse;

#[derive(Debug, Serialize)]
pub struct CreatePostPayload {
    pub category_id: u64,
    pub title: String,
    pub description: Option<String>,
    pub body: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Category {
    pub id: u64,
    pub name: String,
    #[allow(unused)]
    pub description: Option<String>,
}

#[function_component(CreatePost)]
pub fn post_form() -> Html {
    let title_value: UseStateHandle<String> = use_state(String::new);
    let description_value: UseStateHandle<String> = use_state(String::new);
    let body_value: UseStateHandle<String> = use_state(String::new);
    let category_value: UseStateHandle<Option<u64>> = use_state(|| None);

    let category_options = use_state(|| Vec::<Category>::with_capacity(10));

    let on_title_change = {
        let cloned_title_value = title_value.clone();

        Callback::from(move |evt: Event| {
            let target = evt
                .target()
                .expect("Event should have a target when dispatched");

            cloned_title_value.set(target.unchecked_into::<HtmlInputElement>().value());
        })
    };

    let on_description_change = {
        let cloned_description_value = description_value.clone();

        Callback::from(move |evt: Event| {
            let target = evt
                .target()
                .expect("Event should have a target when dispatched");

            cloned_description_value.set(target.unchecked_into::<HtmlInputElement>().value());
        })
    };

    let on_body_change = {
        let cloned_body_value = body_value.clone();

        Callback::from(move |evt: Event| {
            let target = evt
                .target()
                .expect("Event should have a target when dispatched");

            cloned_body_value.set(target.unchecked_into::<HtmlInputElement>().value());
        })
    };

    let on_category_change = {
        let cloned_category_value = category_value.clone();

        Callback::from(move |evt: Event| {
            let target = evt
                .target()
                .expect("Event should have a target when dispatched");

            let value_u64: u64 = target
                .unchecked_into::<HtmlInputElement>()
                .value()
                .parse()
                .unwrap();

            cloned_category_value.set(Some(value_u64));
        })
    };

    let on_ok = {
        let cloned_title_value = title_value.clone();
        let cloned_description_value = description_value.clone();
        let cloned_body_value = body_value.clone();
        let cloned_category_value = category_value.clone();

        Callback::from(move |_| {
            // category_id - Required
            let category_id = cloned_category_value.expect("category_id must be required");

            // Prepare the creation payload
            let payload = CreatePostPayload {
                category_id,
                title: (*cloned_title_value).clone(),
                description: Some((*cloned_description_value).clone()),
                body: Some((*cloned_body_value).clone()),
            };

            wasm_bindgen_futures::spawn_local(async move {
                // TODO Process the request error
                Request::post("http://127.0.0.1:9000/posts")
                    .json(&payload)
                    .unwrap()
                    .send()
                    .await
                    .unwrap();
            });
        })
    };

    // Fetch the categories for selection element
    {
        let cloned_category_options = category_options.clone();

        use_effect_with((), move |_| {
            let cloned_category_options = cloned_category_options.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let categories: Vec<Category> = Request::get("http://127.0.0.1:9000/categories")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .map(|reply: OhMyResponse<Vec<Category>>| reply.data)
                    .unwrap();

                cloned_category_options.set(categories);
            })
        })
    }

    html! {
        <>
            <div class="space-y-12">
                <div class="pb-12">
                    <div class="grid grid-cols-1 gap-x-6 gap-y-8 sm:grid-cols-6">
                        <div class="sm:col-span-4">
                            <label for="title" class="block text-sm font-medium leading-6 text-gray-900">{"Title"}</label>
                            <div class="mt-2">
                                <div class="flex ring-1 ring-inset ring-gray-300 sm:max-w-md">
                                    <input type="text" name="username" id="username" autocomplete="title" class="block flex-1 border-0 bg-transparent py-1 pl-1 text-gray-900 placeholder:text-gray-400" placeholder="Input post title" />
                                </div>
                            </div>
                        </div>
                    </div>

                <div class="sm:col-span-3">
                <label for="country" class="block text-sm font-medium leading-6 text-gray-900">{"Country"}</label>
                <div class="mt-2">
                    <select id="country" name="country" autocomplete="country-name" class="block w-full rounded-md border-0 py-1.5 text-gray-900 shadow-sm ring-1 ring-inset ring-gray-300 focus:ring-2 focus:ring-inset focus:ring-indigo-600 sm:max-w-xs sm:text-sm sm:leading-6">
                    </select>
                </div>
                </div>
                </div>
            </div>


            <div>
                <input
                    onchange={on_title_change}
                    name="title"
                    type="text"
                    class="border"
                />
            </div>

            <div>
                <input
                    onchange={on_description_change}
                    name="description"
                    class="border"
                />
            </div>

            <div>
                <input
                    onchange={on_body_change}
                    name="body"
                    class="border"
                />
            </div>
            <div>
                <select onchange={on_category_change}>
                    {
                        (*category_options).iter().map(|category| {
                            html! {
                                <option key={category.id} value={category.id.to_string()}>
                                    {&category.name}
                                </option>
                            }
                        }).collect::<Html>()
                    }
                </select>
            </div>
            <button
                onclick={on_ok}
            >{"Ok"}</button>
        </>
    }
}
