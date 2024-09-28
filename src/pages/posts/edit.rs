use gloo_net::http::Request;
use pulldown_cmark::{Options, Parser};
use serde::Deserialize;
use web_sys::{wasm_bindgen::JsCast, HtmlInputElement, Node};
use yew::prelude::*;

#[derive(Debug, Deserialize)]
pub struct NormalReply<T> {
    pub message: String,
    data: T,
}

#[derive(Debug, Deserialize, Default)]
pub struct PostDetail {
    id: u64,
    category_id: u64,
    category_name: String,
    title: String,
    description: Option<String>,
    body: Option<String>,
    created_at: Option<chrono::DateTime<chrono::Local>>,
    updated_at: Option<chrono::DateTime<chrono::Local>>,
}

#[derive(Properties, PartialEq)]
pub struct EditPostProps {
    pub id: u64,
}

#[function_component(EditPost)]
pub fn edit_post(props: &EditPostProps) -> Html {
    let post_detail: UseStateHandle<PostDetail> = use_state(Default::default);
    let editor_content = use_state(String::new);

    let on_title_change = {
        // let
        Callback::from(move |_| {})
    };

    let on_editor_change = {
        let cloned_editor_content = editor_content.clone();

        Callback::from(move |evt: Event| {
            let target = evt
                .target()
                .expect("Event should have a target when dispatched");
            let value = target.unchecked_into::<HtmlInputElement>().value();
            cloned_editor_content.set(value);
        })
    };

    {
        let cloned_post_detail = post_detail.clone();
        let id = props.id.clone();

        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let post_detail = Request::get(&format!("http://127.0.0.1:9000/posts/{}", id))
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .map(|reply: NormalReply<PostDetail>| reply.data)
                    .unwrap();

                cloned_post_detail.set(post_detail);
            });
        });
    }

    html! {
        <>
            <div>
                <label for="title">{"Title"}</label>
                <textarea
                    onchange={on_title_change}
                    value={(*post_detail).title.clone()}
                    id="title"
                    class="w-full border p-1"
                />
            </div>
            <div>
                <h1>{"Content"}</h1>
                <div class="w-full flex min-h-32">
                    <div class="flex-1">
                        <textarea
                            onchange={on_editor_change}
                            value={(*post_detail).body.clone()}
                            id="body"
                            class="w-full h-full border p-1"
                        />
                    </div>
                    <div class="flex-1">
                        <article class="prose w-full h-full border">
                            {markdown_to_html(&*editor_content)}
                        </article>
                    </div>
                </div>
            </div>
        </>
    }
}

fn markdown_to_html(input: impl AsRef<str>) -> Html {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    options.insert(Options::ENABLE_SMART_PUNCTUATION);
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(input.as_ref(), options);
    let mut html_output = String::new();

    pulldown_cmark::html::push_html(&mut html_output, parser);

    let div_wrapper = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .create_element("div")
        .unwrap();

    div_wrapper.set_inner_html(&html_output);

    let node: Node = div_wrapper.into();

    Html::VRef(node)
}
