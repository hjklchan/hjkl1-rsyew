use crate::router::Router;
use gloo::console::log;
use gloo_net::http::Request;
use pulldown_cmark::{Options, Parser};
use serde::Deserialize;
use stylist::yew::styled_component;
use yew::prelude::*;
use yew_router::components::Link;

#[derive(Debug, Deserialize)]
pub struct NormalReply<T> {
    pub message: String,
    data: T,
}

#[derive(Debug, Deserialize, Default)]
pub struct Post {
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
pub struct PostDetailProps {
    pub id: u64,
}

#[styled_component(PostDetail)]
pub fn post_detail(props: &PostDetailProps) -> Html {
    let post: UseStateHandle<Post> = use_state(Default::default);

    {
        let id = props.id.clone();
        let cloned_post = post.clone();

        use_effect_with((), move |_| {
            let cloned_post = cloned_post.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let fetched_post = Request::get(&format!("http://127.0.0.1:9000/posts/{}", id))
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .map(|reply: NormalReply<Post>| reply.data)
                    .unwrap();

                cloned_post.set(fetched_post);
            });
        });
    }

    html! {
        <>
            <div class="flex w-full my-2">
                <div class="hidden lg:flex flex-none bg-[#F2F2F2] w-64 mr-8">
                    <div>{"More posts..."}</div>
                </div>
                <div class="flex-1">
                    <h1 class="text-2xl text-gray-600 border-b-2 overflow-hidden border-gray-600 pb-2">
                        // Category
                        <Link<Router> to={Router::Posts} classes="text-[#369]">
                            {format!("[{}] ", &post.category_name)}
                        </Link<Router>>
                        // Title
                        {&post.title}
                        // Created At
                        <span class="text-base pl-2">
                        {
                            post
                                .created_at
                                .map(|updated_at| updated_at.format("%Y/%m/%d").to_string())
                                .unwrap_or("N/A".to_string())
                        }
                        </span>
                    </h1>
                    <div>
                        <article class="my-8 break-words">
                            if let Some(body) = &post.body {
                                {Html::from_html_unchecked(make_html_str(body).into())}
                            } else {
                                <em class="">{"No content..."}</em>
                            }
                        </article>
                    </div>
                </div>
            </div>
        </>
    }
}

fn make_html_str(input: impl AsRef<str>) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);

    let parser = Parser::new_ext(input.as_ref(), options);
    let mut html_output = String::new();

    pulldown_cmark::html::push_html(&mut html_output, parser);

    html_output
}
