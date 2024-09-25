use super::Form;
use gloo_net::http::Request;
use serde::Serialize;
use yew::{function_component, html, use_state, Callback, Html, Properties, UseStateHandle};

#[derive(Debug, Clone, PartialEq)]
pub struct Category {
    pub id: u64,
    pub name: String,
    pub children: Option<Vec<Category>>,
}

/**
 * CreatePayload
 *
 * @deprecated No need to bubble up to the upper level(Post component) to do the processing again
 */
#[derive(Debug, Serialize)]
pub struct CreatePayload {
    pub name: String,
    // pub description: Option<String>,
}

const CATEGORY_API: &'static str = "http://127.0.0.1:9000/categories";

#[derive(Clone, Properties, PartialEq)]
pub struct Category2Props {
    // TODO Using Rc to optimise the item of props?
    pub items: Option<Vec<Category>>,
    #[prop_or_default]
    pub on_select: Callback<Option<u64>>,
    /**
     * on_create
     *
     * @deprecated No need to bubble up to the upper level(Post component) to do the processing again
     */
    #[prop_or_default]
    pub on_create: Callback<CreatePayload>,
    #[prop_or_default]
    pub on_created: Callback<(u64, String)>,
}

#[function_component(Category2)]
pub fn category2(props: &Category2Props) -> Html {
    let current_selected: UseStateHandle<Option<u64>> = use_state(|| None);
    let form_visible: UseStateHandle<bool> = use_state(|| false);

    let on_category_click = {
        let cloned_on_select = props.on_select.clone();
        let cloned_current_selected = current_selected.clone();

        Callback::from(move |category_id: Option<u64>| {
            // Prevent duplicate selection of category
            if *cloned_current_selected == category_id {
                return;
            }

            cloned_current_selected.set(category_id);
            cloned_on_select.emit(category_id);
        })
    };

    let on_create_click = {
        let cloned_form_visible = form_visible.clone();

        Callback::from(move |_| {
            cloned_form_visible.set(!*cloned_form_visible);
        })
    };

    let fetch_create_category = {
        let cloned_form_visible = form_visible.clone();
        let cloned_on_created = props.on_created.clone();

        Callback::from(move |name: String| {
            let cloned_form_visible = cloned_form_visible.clone();
            let cloned_on_created = cloned_on_created.clone();

            let request_url = format!("{}", CATEGORY_API);

            wasm_bindgen_futures::spawn_local(async move {
                let is_ok = Request::post(&request_url)
                    .json(&CreatePayload { name: name.clone() })
                    .unwrap()
                    .send()
                    .await
                    .unwrap()
                    .ok();

                if is_ok {
                    cloned_form_visible.set(false);
                    cloned_on_created.emit((0, name));
                }
            });
        })
    };

    let active_class: Callback<bool, &str> = Callback::from(move |b| {
        if b {
            "p-1 border border-[#369] text-[#369] bg-[#E5EDF2]"
        } else {
            "p-1 border border-[#CDCDCD] text-[#333]"
        }
    });

    let is_default: Callback<(), bool> = {
        let cloned_current_selected = current_selected.clone();

        Callback::from(move |_| {
            cloned_current_selected
                .map(|value| value == 0)
                .unwrap_or(true)
        })
    };

    html! {
        <>
            <ul class="flex flex-wrap items-center my-0 text-xs">
                if let Some(items) = &props.items {
                    // Default category
                    // - Query all posts
                    <li class="float-left pr-2">
                        <button
                            onclick={
                                {
                                    let cloned_on_category_click = on_category_click.clone();
                                    move |_| cloned_on_category_click.emit(None)
                                }
                            }
                            class={active_class.emit(is_default.emit(()))}
                        >
                            {"Default"}
                        </button>
                    </li>
                    {
                        items
                            .iter()
                            .map(|item| {
                                let active_class = {
                                    let b: bool = current_selected
                                        .map(|selected_id| selected_id == item.id)
                                        .unwrap_or(false);

                                    active_class.emit(b)
                                };

                                html! {
                                    <li key={item.id} class="float-left pr-2 my-1">
                                        <button
                                            onclick={
                                                {
                                                    let cloned_on_category_click = on_category_click.clone();
                                                    let category_id = item.id;

                                                    move |_| cloned_on_category_click.emit(Some(category_id))
                                                }
                                            }
                                            class={active_class}
                                        >
                                            {&item.name}
                                        </button>
                                    </li>
                                }
                            })
                            .collect::<Html>()
                    }
                }
                if *form_visible {
                    <li class="float-left pr-2">
                        <Form
                            // handle form closing
                            on_close={
                                let cloned_form_visible = form_visible.clone();
                                Callback::from(move |_| cloned_form_visible.set(false))
                            }

                            // Handle form submitted
                            on_ok={
                                let cloned_fetch_create_category = fetch_create_category.clone();
                                Callback::from(move |name: String| cloned_fetch_create_category.emit(name))
                            }
                        />
                    </li>
                } else {
                    <li class="float-left pr-2">
                        <button
                            onclick={on_create_click}
                            class="p-1 border border-[#CDCDCD] text-[#333] hover:border-[#369] hover:text-[#369] hover:bg-[#E5EDF2]"
                        >
                            {"＋Create"}
                        </button>
                    </li>
                }
            </ul>
        </>
    }
}
