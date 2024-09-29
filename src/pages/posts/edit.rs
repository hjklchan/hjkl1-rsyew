use serde::Deserialize;
use yew::prelude::*;

#[derive(Debug, Deserialize, Default, PartialEq)]
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
    let old_post_detail: UseStateHandle<PostDetail> = use_state_eq(Default::default);
    #[allow(unused)]
    let title_field: UseStateHandle<String> = use_state(Default::default);
    let body_field = use_state(String::new);

    #[allow(unused)]
    let on_editor_input = {
        let cloned_body_field = body_field.clone();

        Callback::from(move |value| {
            cloned_body_field.set(value);
        })
    };

    html! {
        <div class="space-y-1">
            <div>
                <label for="title">{"Title"}</label>
                <input
                    // onchange={on_title_change}
                    value={(*old_post_detail).title.clone()}
                    id="title"
                    class="w-full border p-1"
                />
            </div>
            <div>
                <h1>{"Content"}</h1>
                <div class="w-full flex min-h-32">
                    // Editor
                    <div class="flex-1">
                    </div>
                </div>
            </div>
            <div>
                <button class="border border-[#369] text-xs p-1">{"Save"}</button>
            </div>
        </div>
    }
}
