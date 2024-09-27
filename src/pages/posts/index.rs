use super::jsondata as post_jsondata;
use crate::components::category2::{
    jsondata as category_jsondata,
    Category,
    Category2,
};
use super::RowForm;

use crate::components::icons::{Chat1, Eye1};
use crate::router::Router;
use gloo::console::log;
use gloo_net::http::Request;
use serde::Deserialize;
use yew::{
    function_component, html, use_effect_with, use_state, Callback, Html, Suspense,
    UseStateHandle,
};
use yew_router::prelude::Link;

#[derive(Debug, Deserialize)]
pub struct CategoryItem {
    pub id: u64,
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ListReply<T> {
    pub data: Vec<T>,
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct PaginationData<T> {
    pub items: Vec<T>,
    pub page_size: u64,
    pub has_prev: bool,
    pub has_next: bool,
    pub total: u64,
}

#[derive(Debug, Deserialize)]
pub struct PaginationReply<T> {
    pub message: String,
    pub data: PaginationData<T>,
}

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct Post {
    pub id: u64,
    pub category_id: u64,
    pub category_name: String,
    pub title: String,
    pub description: Option<String>,
    pub created_at: Option<chrono::DateTime<chrono::Local>>,
    pub updated_at: Option<chrono::DateTime<chrono::Local>>,
}

#[derive(Debug, Default, PartialEq)]
struct Pagination {
    pub has_next: bool,
    pub has_prev: bool,
}

#[function_component(Posts)]
pub fn posts() -> Html {
    let posts_url = "http://127.0.0.1:9000/posts";
    
    let categories: UseStateHandle<Vec<Category>> = use_state(Vec::new);
    let posts: UseStateHandle<Vec<Post>> = use_state(Vec::new);
    let pagination: UseStateHandle<Pagination> = use_state(Default::default);
    let current_page: UseStateHandle<u32> = use_state(|| 1);
    let current_category: UseStateHandle<Option<u64>> = use_state(|| None);
    let category_form_visible: UseStateHandle<bool> = use_state(|| false);
    let post_row_form_visible: UseStateHandle<bool> = use_state(|| false);

    // Fetch categories
    // - onload
    {
        let cloned_categories = categories.clone();

        use_effect_with((), move |_| {
            log!("fetching categories");

            let cloned_categories = cloned_categories.clone();

            wasm_bindgen_futures::spawn_local(async move {
                let categories = Request::get("http://127.0.0.1:9000/categories")
                    .send()
                    .await
                    .map_err(|err| {
                        cloned_categories.set(category_jsondata::make_fake_categories());
                        err
                    })
                    .unwrap()
                    .json()
                    .await
                    .map(|reply: ListReply<CategoryItem>| {
                        reply
                            .data
                            .iter()
                            .map(|item| Category {
                                id: item.id,
                                name: item.name.clone(),
                                children: None,
                            })
                            .collect::<Vec<Category>>()
                    })
                    .unwrap();

                cloned_categories.set(categories);
            });

            || {}
        });
    }

    // Fetch posts
    // - onload
    {
        let cloned_posts = posts.clone();
        let cloned_pagination = pagination.clone();
        let cloned_current_page = current_page.clone();

        // FIXME 通过监听当前分类和当前分页获取文章的问题
        // - 不能同时监听当前分类和当前分页
        // - 当更换页码后更换分类会出现当前页码数仍然保留
        use_effect_with(
            (*current_category, *cloned_current_page),
            move |(category_id, page)| {
                // Cloned state
                let cloned_posts = cloned_posts.clone();
                let cloned_pagination = cloned_pagination.clone();

                let category_query = category_id
                    .map(|id| format!("&category_id={}", id))
                    .unwrap_or_else(String::new);

                let request_url = format!("{}?page={}{}", posts_url, page, category_query);

                wasm_bindgen_futures::spawn_local(async move {
                    let fetched_posts = Request::get(&request_url)
                        .send()
                        .await
                        .map_err(|err| {
                            // 更新假数据到 posts 状态中
                            cloned_posts.set(post_jsondata::make_fake_posts());
                            err
                        })
                        .unwrap()
                        .json()
                        .await
                        .map(|reply: PaginationReply<Post>| {
                            // NOTE Pagination
                            let new_pagination = Pagination {
                                has_next: reply.data.has_next,
                                has_prev: reply.data.has_prev,
                            };

                            cloned_pagination.set(new_pagination);

                            reply.data.items
                        })
                        .map_err(|err| err)
                        .unwrap();

                    cloned_posts.set(fetched_posts);
                });

                || {}
            },
        );
    }

    let on_quick_create = {
        let cloned_post_row_form_visible = post_row_form_visible.clone();

        Callback::from(move |_| {
            cloned_post_row_form_visible.set(true);
        })
    };

    let on_quick_create_close = {
        let clone_post_row_form_visible = post_row_form_visible.clone();
        // Close the row form
        Callback::from(move |_| clone_post_row_form_visible.set(false))
    };

    let on_create_category = {
        let cloned_categories = categories.clone();
        let cloned_category_form_visible = category_form_visible.clone();
        Callback::from(move |(new_id, name)| {
            log!("on_create_category is rendered");

            cloned_category_form_visible.set(!*cloned_category_form_visible);

            let mut old_categories = (*cloned_categories).clone();
            old_categories.push(Category {
                id: new_id,
                name,
                children: None,
            });
            cloned_categories.set(old_categories);
        })
    };

    let on_next_page = {
        let cloned_current_page = current_page.clone();

        Callback::from(move |_| {
            log!("on_next_page is rendered");

            cloned_current_page.set(*cloned_current_page + 1);
        })
    };

    let on_prev_page = {
        let cloned_current_page = current_page.clone();

        Callback::from(move |_| {
            log!("on_prev_page is rendered");

            cloned_current_page.set(*cloned_current_page - 1);
        })
    };

    html! {
        <>
            <Suspense fallback={html!(<em>{"Loading..."}</em>)}>
                <Category2
                    items={(*categories).clone()}
                    on_select={move |id: Option<u64>| current_category.set(id)}
                    // @deprecated
                    // on_create={move |_| on_create_category.emit(())}
                    on_created={move |name| on_create_category.emit(name)}
                />
            </Suspense>
            // Category
            // Posts - Header
            <div class="mt-4">
                <table
                    class="w-full table-fixed"
                    cellspacing={0}
                    cellpadding={0}
                >
                    <tbody>
                        <tr class="border-b border-[#C2D5E3] bg-[#F2F2F2] border-[#C2D5E3] text-xs">
                            <td
                                colspan={2}
                                class="text-left pl-2 py-3 space-x-3"
                            >
                                <div class="inline-block">
                                    <button
                                        onclick={on_quick_create}
                                        class="text-[#333] hover:text-[#369] border-b border-[#333]"
                                    >
                                        {"Quick Create"}
                                    </button>
                                </div>
                                <div class="inline-block space-x-1">
                                    <input
                                        type="checkbox"
                                        // TODO
                                        // defaultChecked
                                        // onChange={onNewTab}
                                    />
                                    <label>{"New Tab"}{"(Ctrl + Click)"}</label>
                                </div>
                                <button class="text-[#369]">
                                    {"All"}
                                </button>
                                <button class="text-[#369]">
                                    {"Newest"}
                                </button>
                                <button class="text-[#369]">
                                    {"Popular"}
                                </button>
                            </td>
                            // TODO Hide by special device
                            <td class="hidden lg:table-cell w-28">{"Author"}</td>
                            <td class="hidden lg:table-cell w-24">
                                <div class="flex space-x-1">
                                    <Eye1 classes="w-3" />
                                    <span>{"/"}</span>
                                    <Chat1 classes="w-3" />
                                </div>
                            </td>
                            <td class="hidden lg:table-cell w-28">{"Last Updated"}</td>
                        </tr>
                        if *post_row_form_visible {
                            <tr>
                                <td class="px-1 py-2">
                                    <RowForm on_close={on_quick_create_close} />
                                </td>
                                <td class="hidden lg:table-cell w-28"></td>
                                <td class="hidden lg:table-cell w-24"></td>
                                <td class="hidden lg:table-cell w-28"></td>
                            </tr>
                        }
                    </tbody>
                </table>
                // Posts - List
                <table
                    class="table-fixed w-full text-sm text-[#334]"
                    cellspacing={0}
                    cellpadding={0}
                >
                    // Using local css
                    <tbody class={""}>
                        {
                            posts.iter().map(|post| {
                                let updated_at_str: Option<String> = post
                                .updated_at
                                .map(|updated_at| updated_at.format("%Y/%m/%d").to_string());

                                html! {
                                    <tr class="table-row align-middle hover:bg-[#F2F2F2] border-b border-[#C2D5E3]">
                                        <td class="w-5 px-1 py-1">
                                            // TODO
                                            // Chatting
                                            <Chat1 classes="text-orange-700 w-4" />
                                            // Locked
                                            // <Lock1 classes="text-red-500 w-4" />
                                        </td>
                                        <td class="py-1">
                                            // Link component
                                            <span class="hover:cursor-pointer text-[#369] pl-1 pr-2">
                                                {format!("[{}]", post.category_name)}
                                            </span>
                                            <Link<Router>
                                                to={Router::PostDetail { id: post.id }}
                                                classes="text-[#333] hover:cursor-pointer hover:border-b border-[#333]"
                                                // TODO
                                                // {...newTabProps}
                                            >
                                                {&post.title}
                                            </Link<Router>>
                                        </td>
                                        <td class="hidden lg:table-cell w-28">
                                            <cite>
                                                <button
                                                    class="hover:cursor-pointer text-[#369]"
                                                    title="(TODO)"
                                                >
                                                    {"(TODO)"}
                                                </button>
                                            </cite>
                                        </td>
                                        <td class="hidden lg:table-cell w-24">{"1.2k / 5k"}</td>
                                        <td class="hidden lg:table-cell w-28">
                                            if let Some(updated_at) = updated_at_str {
                                                {updated_at}
                                            } else {
                                                {"N/A"}
                                            }
                                        </td>
                                    </tr>
                                }
                            }).collect::<Html>()
                        }

                    </tbody>
                </table>
                <div class="space-x-2 my-2 text-sm">
                    if pagination.has_prev {
                        <button onclick={on_prev_page}>{"<< Prev"}</button>
                        <span>{"|"}</span>
                    }
                    if pagination.has_next {
                        <button
                            onclick={on_next_page}
                        >
                            {"Next >>"}
                        </button>
                    }
                </div>
            </div>
        </>
    }
}
