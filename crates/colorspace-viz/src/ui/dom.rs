use wasm_bindgen::JsCast;
use web_sys::Document;
use web_sys::Element;
use web_sys::HtmlElement;
use web_sys::Node;

pub fn el(document: &Document, tag: &str) -> Element {
    document.create_element(tag).expect("create_element")
}

pub fn el_attr(document: &Document, tag: &str, attr: &str, val: &str) -> Element {
    let e = el(document, tag);
    e.set_attribute(attr, val).expect("set_attribute");
    e
}

pub fn append(parent: &Node, child: &Node) {
    parent.append_child(child).expect("append_child");
}

pub fn append_text(parent: &Element, text: &str) {
    let document = parent.owner_document().expect("owner_document");
    let node = document.create_text_node(text);
    parent.append_child(&node).expect("append text");
}

pub fn inject_styles(document: &Document, css: &str) {
    let style = el(document, "style");
    style.set_inner_html(css);
    let head = document.head().expect("head");
    append(&head, &style);
}

pub fn as_html(el: &Element) -> &HtmlElement {
    el.dyn_ref::<HtmlElement>().expect("not HtmlElement")
}
