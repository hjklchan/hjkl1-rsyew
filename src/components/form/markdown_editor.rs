use gloo::console::log;
use web_sys::{wasm_bindgen::JsCast, HtmlInputElement, Node};
use yew::prelude::*;

use crate::utils::convert::markdown_to_html;

#[derive(Properties, PartialEq)]
pub struct MarkdownEditorProps {
    #[prop_or("".into())]
    pub content: AttrValue,
    #[prop_or_default]
    pub on_input: Callback<String>,
}

#[function_component(MarkdownEditor)]
pub fn markdown_editor(props: &MarkdownEditorProps) -> Html {
    let content = use_state(|| props.content.clone());

    let on_content_input = {
        // let cloned_content = content.clone();
        let cloned_on_input = props.on_input.clone();

        Callback::from(move |evt: InputEvent| {
            let value = evt
                .target()
                .unwrap()
                .unchecked_into::<HtmlInputElement>()
                .value();

            // cloned_content.set(value.clone().into());
            cloned_on_input.emit(value);
        })
    };

    html! {
        <div class="flex">
            // Editor
            <div class="flex-1">
                <textarea
                    value={&*content}
                    oninput={on_content_input}
                    class="border w-full"
                />
            </div>
            // Preview
            <div class="flex-1">
                <article class="prose w-full h-full border p-1">
                    {markdown_to_html(&*content)}
                </article>
            </div>
        </div>
    }
}
