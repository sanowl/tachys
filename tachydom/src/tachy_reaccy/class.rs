use crate::{html::class::IntoClass, renderer::DomRenderer};
use tachy_reaccy::RenderEffect;

impl<F, C, R> IntoClass<R> for F
where
    F: Fn() -> C + 'static,
    C: IntoClass<R> + 'static,
    C::State: 'static,
    R: DomRenderer,
    R::ClassList: 'static,
    R::Element: Clone + 'static,
{
    type State = RenderEffect<C::State>;

    fn to_html(self, class: &mut String) {
        let value = self();
        value.to_html(class);
    }

    fn hydrate<const FROM_SERVER: bool>(self, el: &R::Element) -> Self::State {
        // TODO FROM_SERVER vs template
        let el = el.clone();
        RenderEffect::new(move |prev| {
            let value = self();
            if let Some(mut state) = prev {
                value.rebuild(&mut state);
                state
            } else {
                value.hydrate::<FROM_SERVER>(&el)
            }
        })
    }

    fn build(self, el: &R::Element) -> Self::State {
        let el = el.to_owned();
        RenderEffect::new(move |prev| {
            let value = self();
            if let Some(mut state) = prev {
                value.rebuild(&mut state);
                state
            } else {
                value.build(&el)
            }
        })
    }

    fn rebuild(self, state: &mut Self::State) {}
}

impl<F, R> IntoClass<R> for (&'static str, F)
where
    F: Fn() -> bool + 'static,
    R: DomRenderer,
    R::ClassList: 'static,
    R::Element: Clone,
{
    type State = RenderEffect<bool>;

    fn to_html(self, class: &mut String) {
        let (name, f) = self;
        let include = f();
        if include {
            <&str as IntoClass<R>>::to_html(name, class);
        }
    }

    fn hydrate<const FROM_SERVER: bool>(self, el: &R::Element) -> Self::State {
        // TODO FROM_SERVER vs template
        let (name, f) = self;
        let class_list = R::class_list(el);
        RenderEffect::new(move |prev| {
            let include = f();
            if Some(include) != prev {
                if include {
                    R::add_class(&class_list, name);
                } else {
                    R::remove_class(&class_list, name);
                }
            }
            include
        })
    }

    fn build(self, el: &R::Element) -> Self::State {
        let (name, f) = self;
        let class_list = R::class_list(el);
        RenderEffect::new(move |prev| {
            let include = f();
            if Some(include) != prev {
                if include {
                    R::add_class(&class_list, name);
                } else {
                    R::remove_class(&class_list, name);
                }
            }
            include
        })
    }

    fn rebuild(self, state: &mut Self::State) {}
}
