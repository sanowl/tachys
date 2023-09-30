use super::{CastFrom, DomRenderer, Renderer};
use crate::{
    dom::document,
    html::element::{CreateElement, ElementType},
    ok_or_debug, or_debug,
    view::Mountable,
};
use once_cell::unsync::Lazy;
use wasm_bindgen::{intern, JsCast, JsValue};
use web_sys::{
    CssStyleDeclaration, DocumentFragment, DomTokenList, Element, HtmlElement,
    Node, Text,
};

pub struct Dom;

impl Renderer for Dom {
    type Node = Node;
    type Text = Text;
    type Element = Element;

    fn create_element<E: ElementType + CreateElement<Dom>>() -> Self::Element {
        E::create_element()
    }

    fn create_text_node(text: &str) -> Self::Text {
        document().create_text_node(text)
    }

    fn set_text(node: &Self::Text, text: &str) {
        node.set_data(text);
    }

    fn set_attribute(node: &Self::Element, name: &str, value: &str) {
        or_debug!(
            node.set_attribute(intern(name), value),
            node,
            "setAttribute"
        );
    }

    fn remove_attribute(node: &Self::Element, name: &str) {
        or_debug!(node.remove_attribute(intern(name)), node, "removeAttribute");
    }

    fn insert_node(
        parent: &Self::Element,
        new_child: &Self::Node,
        anchor: Option<&Self::Node>,
    ) {
        ok_or_debug!(
            parent.insert_before(new_child, anchor),
            parent,
            "insertNode"
        );
    }

    fn remove_node(
        parent: &Self::Element,
        child: &Self::Node,
    ) -> Option<Self::Node> {
        ok_or_debug!(parent.remove_child(child), parent, "removeNode")
    }

    fn remove(node: &Self::Node) {
        node.unchecked_ref::<Element>().remove();
    }

    fn get_parent(node: &Self::Node) -> Option<Self::Node> {
        node.parent_node()
    }

    fn first_child(node: &Self::Node) -> Option<Self::Node> {
        node.first_child()
    }

    fn next_sibling(node: &Self::Node) -> Option<Self::Node> {
        node.next_sibling()
    }

    fn replace_node(old: &Self::Node, new: &Self::Node) {
        or_debug!(
            old.unchecked_ref::<Element>().replace_with_with_node_1(new),
            old,
            "replaceWith"
        );
    }
}

impl DomRenderer for Dom {
    type Event = JsValue;
    type ClassList = DomTokenList;
    type CssStyleDeclaration = CssStyleDeclaration;

    fn add_event_listener(
        el: &Self::Element,
        name: &str,
        cb: Box<dyn FnMut(Self::Event)>,
    ) {
        let cb = wasm_bindgen::closure::Closure::wrap(cb).into_js_value();
        el.add_event_listener_with_callback(name, cb.as_ref().unchecked_ref());
    }

    fn class_list(el: &Self::Element) -> Self::ClassList {
        el.class_list()
    }

    fn add_class(list: &Self::ClassList, name: &str) {
        or_debug!(list.add_1(name), list.unchecked_ref(), "add()");
    }

    fn remove_class(list: &Self::ClassList, name: &str) {
        or_debug!(list.remove_1(name), list.unchecked_ref(), "remove()");
    }

    fn style(el: &Self::Element) -> Self::CssStyleDeclaration {
        el.unchecked_ref::<HtmlElement>().style()
    }

    fn set_css_property(
        style: &Self::CssStyleDeclaration,
        name: &str,
        value: &str,
    ) {
        or_debug!(
            style.set_property(name, value),
            style.unchecked_ref(),
            "setProperty"
        );
    }
}

impl<E: ElementType> CreateElement<Dom> for E {
    fn create_element() -> Element {
        thread_local! {
            static ELEMENT: Lazy<Element> = Lazy::new(|| {
                document().create_element(stringify!($tag)).unwrap()
            });
        }
        ELEMENT.with(|e| e.clone_node()).unwrap().unchecked_into()
    }
}

impl Mountable<Dom> for Node {
    fn unmount(&mut self) {
        todo!()
    }

    fn mount(&self, parent: &Element, marker: Option<&Node>) {
        Dom::insert_node(parent, self, marker);
    }
}

impl Mountable<Dom> for Text {
    fn unmount(&mut self) {
        todo!()
    }

    fn mount(&self, parent: &Element, marker: Option<&Node>) {
        Dom::insert_node(parent, self, marker);
    }
}

impl Mountable<Dom> for Element {
    fn unmount(&mut self) {
        todo!()
    }

    fn mount(&self, parent: &Element, marker: Option<&Node>) {
        Dom::insert_node(parent, self, marker);
    }
}

impl Mountable<Dom> for DocumentFragment {
    fn unmount(&mut self) {
        todo!()
    }

    fn mount(&self, parent: &Element, marker: Option<&Node>) {
        Dom::insert_node(parent, self, marker);
    }
}

impl CastFrom<Node> for Text {
    fn cast_from(node: Node) -> Option<Text> {
        node.clone().dyn_into().ok()
    }
}

impl CastFrom<Node> for Element {
    fn cast_from(node: Node) -> Option<Element> {
        node.clone().dyn_into().ok()
    }
}
