use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct FormProps {
    #[prop_or_default]
    pub on_ok: Callback<String>,
    #[prop_or_default]
    pub on_close: Callback<()>,
}

#[function_component(Form)]
pub fn form(props: &FormProps) -> Html {
    let input_node_ref = use_node_ref();

    let on_ok_click = {
        let cloned_input_node_ref = input_node_ref.clone();
        let clone_on_ok = props.on_ok.clone();

        Callback::from(move |_| {
            let input = cloned_input_node_ref.cast::<HtmlInputElement>();
            let value = input
                .map(|html_input_element| html_input_element.value())
                .unwrap_or_default();

            clone_on_ok.emit(value);
        })
    };

    let onblur = {
        let cloned_on_close = props.on_close.clone();
        
        Callback::from(move |_| {
            cloned_on_close.emit(());
        })
    };

    let on_close_click = {
        let cloned_on_close = props.on_close.clone();

        Callback::from(move |_| cloned_on_close.emit(()))
    };

    {
        let cloned_input_node_ref = input_node_ref.clone();

        use_effect_with((), move |_| {
            cloned_input_node_ref
                .cast::<HtmlInputElement>()
                .unwrap()
                .focus()
                .unwrap();
        });
    }

    html! {
        <div class="flex space-x-1">
            // <input class="p-1 border border-[#CDCDCD] focus:border-[#369]" />
            <div class="relative">
                <input
                    ref={input_node_ref}
                    {onblur}
                    type="text"
                    name="price"
                    id="price"
                    class="block w-full border border-[#333] p-1 pr-8 text-gray-900 placeholder:text-gray-400"
                />
                <div class="absolute inset-y-0 right-0 flex items-center">
                    <button
                        onclick={on_close_click}
                        class="h-full border-0 py-0 pl-2 pr-2"
                    >
                        <b class="text-red-900">{"×"}</b>
                    </button>
                </div>
            </div>
            <button
                onclick={on_ok_click}
                class="block text-xs text-green-900"
            >
                {"√"}
            </button>
        </div>
    }
}
