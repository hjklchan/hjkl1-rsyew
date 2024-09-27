use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use web_sys::{wasm_bindgen::JsCast, HtmlInputElement};
use yew::prelude::*;

#[derive(Debug, Deserialize)]
pub struct NormalReply<T> {
    pub message: String,
    pub data: T,
}

#[derive(Debug, Serialize)]
pub struct CreatePostPayload {
    pub category_id: u64,
    pub title: String,
    pub description: Option<String>,
    pub body: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePostReplyData {
    pub new_id: u64,
}

#[derive(Debug, Deserialize)]
pub struct Category {
    pub id: u64,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Properties, PartialEq)]
pub struct RowFormProps {
    #[prop_or_default]
    pub on_close: Callback<()>,
    #[prop_or_default]
    pub on_created: Callback<u64>,
    #[prop_or(false)]
    visible: bool, // @deprecated
}

#[function_component(RowForm)]
pub fn row_form(props: &RowFormProps) -> Html {
    let category_options: UseStateHandle<Vec<Category>> =
        use_state(|| Vec::<Category>::with_capacity(10));
    // Form field - Title
    let title_value: UseStateHandle<String> = use_state(String::new);
    // Form field - Category
    let category_id: UseStateHandle<Option<u64>> = use_state(|| None);

    let on_title_change = {
        let cloned_title_value = title_value.clone();

        Callback::from(move |evt: Event| {
            let target = evt
                .target()
                .expect("Event should have a target when dispatched");

            cloned_title_value.set(target.unchecked_into::<HtmlInputElement>().value());
        })
    };

    let on_category_change = {
        let cloned_category_id = category_id.clone();

        Callback::from(move |evt: Event| {
            let target = evt
                .target()
                .expect("Event should have a target when dispatched");

            let value_u64: u64 = target
                .unchecked_into::<HtmlInputElement>()
                .value()
                .parse()
                .unwrap();

            cloned_category_id.set(Some(value_u64));
        })
    };

    let on_close_click = {
        let cloned_on_close = props.on_close.clone();
        Callback::from(move |_| cloned_on_close.emit(()))
    };

    let on_ok_click = {
        let cloned_category_id = category_id.clone();
        let cloned_title_value = title_value.clone();

        let cloned_on_created = props.on_created.clone();
        // Submit form and returning new id
        Callback::from(move |_| {
            let cloned_on_created = cloned_on_created.clone();
            let category_id = cloned_category_id.expect("category_id must be required");

            // Prepare the creation payload
            let payload = CreatePostPayload {
                category_id,
                title: (*cloned_title_value).clone(),
                description: None,
                body: None,
            };

            wasm_bindgen_futures::spawn_local(async move {
                // TODO Process the request error
                let new_id: u64 = Request::post("http://127.0.0.1:9000/posts")
                    .json(&payload)
                    .unwrap()
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .map(|reply: NormalReply<CreatePostReplyData>| reply.data.new_id)
                    .unwrap();

                cloned_on_created.emit(new_id);
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
                    .map(|reply: NormalReply<Vec<Category>>| reply.data)
                    .unwrap();

                cloned_category_options.set(categories);
            })
        })
    }

    html! {
        <div class="flex space-x-2 text-xs">
            <div class="inline-block space-x-1">
                <label>{"Title"}</label>
                <input onchange={on_title_change} class="border px-1" />
            </div>
            <div class="flex inline-block items-center space-x-1">
                <label>{"Category"}</label>
                <select onchange={on_category_change} class="border">
                    {
                        (*category_options)
                            .iter()
                            .map(|category| {
                                html! {
                                    <option key={category.id} value={category.id.to_string()}>
                                        {&category.name}
                                    </option>
                                }
                            })
                            .collect::<Html>()
                    }
                </select>
            </div>
            <div class="flex items-center inline-block space-x-2">
                <button
                    onclick={on_ok_click}
                    class="text-[#369]"
                >
                    {"Ok"}
                </button>
                <button
                    onclick={on_close_click}
                    class="text-red-500"
                >
                    {"Close"}
                </button>
            </div>
        </div>
    }
}
