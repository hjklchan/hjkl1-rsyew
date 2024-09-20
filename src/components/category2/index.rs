use std::{
    collections::{HashMap, VecDeque},
    iter::Map,
};

use yew::{function_component, html, use_effect, use_state, Callback, Html, Properties};

#[derive(Clone, Debug, PartialEq)]
pub struct Category {
    pub id: u64,
    pub name: String,
    pub children: Option<Vec<Category>>,
}

#[derive(Clone, Properties, PartialEq)]
pub struct Category2Props {
    pub items: Option<Vec<Category>>,
    // pub on_selected: Option<Callback<u64>>,
    // pub on_reset: Option<Callback<()>>,
}

#[function_component(Category2)]
pub fn category2(props: &Category2Props) -> Html {
    let mock_data: Vec<Category> = vec![Category {
        id: 1,
        name: "Backend".to_string(),
        children: Some(vec![
            Category {
                id: 2,
                name: "PHP".to_string(),
                children: Some(vec![
                    Category {
                        id: 3,
                        name: "Laravel".to_string(),
                        children: None,
                    },
                    Category {
                        id: 4,
                        name: "ThinkPHP".to_string(),
                        children: None,
                    },
                ]),
            },
            Category {
                id: 5,
                name: "Frontend".to_string(),
                children: Some(vec![
                    Category {
                        id: 6,
                        name: "Javascript".to_string(),
                        children: None,
                    },
                    Category {
                        id: 7,
                        name: "Typescript".to_string(),
                        children: None,
                    },
                ]),
            },
        ]),
    }];

    // NOTE preselectItems state
    // HashMap<`Level`, Category>
    let preselect_items = use_state(|| {
        let mut m = HashMap::new();
        m.insert(
            1,
            Category {
                id: 0,
                name: String::from("root"),
                children: Some(mock_data.clone()),
            },
        );
        m
    });

    use_effect(move || {
        let inner_preselect_items = preselect_items.clone();
        web_sys::console::log_1(&format!("{:#?} {}", inner_preselect_items, inner_preselect_items.len()).into());
    });

    html! {
        <>
            if preselect_items.len() > 1 {

            } else {

            }
        </>
    }
}
