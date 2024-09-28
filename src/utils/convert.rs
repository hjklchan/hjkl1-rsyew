use yew::prelude::Html;
use pulldown_cmark::{html, Parser, Options};
use web_sys::Node;

pub fn markdown_to_html(raw: impl AsRef<str>) -> Html {
    // 额外功能选项
    let options = Options::all();

    // 实例化解析器
    let parser = Parser::new_ext(raw.as_ref(), options);

    // 最终输出的 HTML 字符串
    let mut html_output = String::with_capacity(30);
    html::push_html(&mut html_output, parser);

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