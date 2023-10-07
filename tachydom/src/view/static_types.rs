use super::{
    Mountable, Position, PositionState, Render, RenderHtml, ToTemplate,
};
use crate::{
    html::attribute::{Attribute, AttributeKey, AttributeValue},
    hydration::Cursor,
    renderer::Renderer,
};
use std::marker::PhantomData;

/// An attribute for which both the key and the value are known at compile time,
/// i.e., as `&'static str`s.
///
/// ```
/// use tachydom::{
///     html::attribute::{Attribute, Type},
///     view::static_types::{static_attr, StaticAttr},
/// };
/// let input_type = static_attr::<Type, "text">();
/// let mut buf = String::new();
/// let mut classes = String::new();
/// let mut styles = String::new();
/// input_type.to_html(&mut buf, &mut classes, &mut styles);
/// assert_eq!(buf, " type=\"text\"");
/// ```
#[derive(Debug)]
pub struct StaticAttr<K: AttributeKey, const V: &'static str> {
    ty: PhantomData<K>,
}

pub fn static_attr<K: AttributeKey, const V: &'static str>() -> StaticAttr<K, V>
{
    StaticAttr { ty: PhantomData }
}

impl<K, const V: &'static str> ToTemplate for StaticAttr<K, V>
where
    K: AttributeKey,
{
    fn to_template(buf: &mut String, _position: &mut Position) {
        buf.push(' ');
        buf.push_str(K::KEY);
        buf.push_str("=\"");
        buf.push_str(V);
        buf.push('"');
    }
}

impl<K, const V: &'static str, R> Attribute<R> for StaticAttr<K, V>
where
    K: AttributeKey,
    R: Renderer,
    R::Element: Clone,
{
    type State = ();

    fn to_html(
        self,
        buf: &mut String,
        _class: &mut String,
        _style: &mut String,
    ) {
        AttributeValue::<R>::to_html(V, K::KEY, buf)
    }

    fn hydrate<const FROM_SERVER: bool>(self, _el: &R::Element) -> Self::State {
    }

    fn build(self, el: &R::Element) -> Self::State {
        R::set_attribute(el, K::KEY, V);
    }

    fn rebuild(self, _state: &mut Self::State) {}
}

#[derive(Debug)]
pub struct Static<const V: &'static str>;

impl<const V: &'static str, R: Renderer> Render<R> for Static<V>
where
    R::Text: Mountable<R>,
{
    type State = Option<R::Text>;

    fn build(self) -> Self::State {
        // a view state has to be returned so it can be mounted
        Some(R::create_text_node(V))
    }

    // This type is specified as static, so no rebuilding is done.
    fn rebuild(self, _state: &mut Self::State) {}
}

impl<const V: &'static str, R> RenderHtml<R> for Static<V>
where
    R: Renderer,
    R::Node: Clone,
    R::Element: Clone,
    R::Text: Mountable<R>,
{
    fn to_html(self, buf: &mut String, position: &PositionState) {
        // add a comment node to separate from previous sibling, if any
        if matches!(position.get(), Position::NextChild | Position::LastChild) {
            buf.push_str("<!>")
        }
        buf.push_str(V)
    }

    fn hydrate<const FROM_SERVER: bool>(
        self,
        cursor: &Cursor<R>,
        position: &PositionState,
    ) -> Self::State {
        if position.get() == Position::FirstChild {
            cursor.child();
        } else {
            cursor.sibling();
        }
        position.set(Position::NextChild);

        // no view state is created when hydrating, because this is static
        None
    }
}

impl<const V: &'static str> ToTemplate for Static<V> {
    fn to_template(buf: &mut String, position: &mut Position) {
        if matches!(*position, Position::NextChild | Position::LastChild) {
            buf.push_str("<!>")
        }
        buf.push_str(V);
        *position = Position::NextChild;
    }
}
