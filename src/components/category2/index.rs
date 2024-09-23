use yew::{function_component, html, use_state, Callback, Html, Properties, UseStateHandle};

#[derive(Clone, Debug, PartialEq)]
pub struct Category {
    pub id: u64,
    pub name: String,
    pub children: Option<Vec<Category>>,
}

#[derive(Clone, Properties, PartialEq)]
pub struct Category2Props {
    pub items: Option<Vec<Category>>,
    #[prop_or_default]
    pub on_select: Callback<Option<u64>>,
}

#[function_component(Category2)]
pub fn category2(props: &Category2Props) -> Html {
    let current_selected: UseStateHandle<Option<u64>> = use_state(|| None);

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
            <ul class="flex flex-wrap items-center my-2 text-xs">
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
                                    <li key={item.id} class="float-left pr-2">
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
            </ul>
        </>
    }
}
