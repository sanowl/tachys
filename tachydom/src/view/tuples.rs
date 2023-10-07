use super::{
    Mountable, Position, PositionState, Render, RenderHtml, Renderer,
    ToTemplate,
};
use crate::hydration::Cursor;

impl<R: Renderer> Render<R> for () {
    type State = ();

    fn build(self) -> Self::State {}

    fn rebuild(self, _state: &mut Self::State) {}
}

impl<R> RenderHtml<R> for ()
where
    R: Renderer,
    R::Node: Clone,
    R::Element: Clone,
{
    fn to_html(&self, _buf: &mut String, _position: &PositionState) {}

    fn hydrate<const FROM_SERVER: bool>(
        self,
        _cursor: &Cursor<R>,
        _position: &PositionState,
    ) -> Self::State {
    }
}

impl<R: Renderer> Mountable<R> for () {
    fn unmount(&mut self) {}

    fn mount(&mut self, _parent: &R::Element, _marker: Option<&R::Node>) {}

    fn insert_before_this(
        &self,
        _parent: &<R as Renderer>::Element,
        _child: &mut dyn Mountable<R>,
    ) -> bool {
        false
    }
}

impl ToTemplate for () {
    fn to_template(_buf: &mut String, _position: &mut Position) {}
}

impl<A: Render<R>, R: Renderer> Render<R> for (A,) {
    type State = A::State;

    fn build(self) -> Self::State {
        self.0.build()
    }

    fn rebuild(self, state: &mut Self::State) {
        self.0.rebuild(state)
    }
}

impl<A, R> RenderHtml<R> for (A,)
where
    A: RenderHtml<R>,
    R: Renderer,
    R::Node: Clone,
    R::Element: Clone,
{
    fn to_html(&self, buf: &mut String, position: &PositionState) {
        self.0.to_html(buf, position);
    }

    fn hydrate<const FROM_SERVER: bool>(
        self,
        cursor: &Cursor<R>,
        position: &PositionState,
    ) -> Self::State {
        self.0.hydrate::<FROM_SERVER>(cursor, position)
    }
}

impl<A: ToTemplate> ToTemplate for (A,) {
    fn to_template(buf: &mut String, position: &mut Position) {
        A::to_template(buf, position)
    }
}

macro_rules! impl_view_for_tuples {
	($first:ident, $($ty:ident),* $(,)?) => {
		impl<$first, $($ty),*, Rndr> Render<Rndr> for ($first, $($ty,)*)
		where
			$first: Render<Rndr>,
			$($ty: Render<Rndr>),*,
			Rndr: Renderer
		{
			type State = ($first::State, $($ty::State,)*);

			fn build(self) -> Self::State {
				paste::paste! {
					let ([<$first:lower>], $([<$ty:lower>],)*) = self;
					(
						[<$first:lower>].build(),
						$([<$ty:lower>].build()),*
					)
				}
			}

			fn rebuild(self, state: &mut Self::State) {
				paste::paste! {
					let ([<$first:lower>], $([<$ty:lower>],)*) = self;
					let ([<view_ $first:lower>], $([<view_ $ty:lower>],)*) = state;
					[<$first:lower>].rebuild([<view_ $first:lower>]);
					$([<$ty:lower>].rebuild([<view_ $ty:lower>]));*
				}
			}
		}

		impl<$first, $($ty),*, Rndr> RenderHtml<Rndr> for ($first, $($ty,)*)
		where
			$first: RenderHtml<Rndr>,
			$($ty: RenderHtml<Rndr>),*,
			Rndr: Renderer,
			Rndr::Node: Clone,
			Rndr::Element: Clone
		{
			fn to_html(&self, buf: &mut String, position: &PositionState) {
				paste::paste! {
					let ([<$first:lower>], $([<$ty:lower>],)* ) = self;
					[<$first:lower>].to_html(buf, position);
					position.set(Position::NextChild);
					$([<$ty:lower>].to_html(buf, position));*
				}
			}

			fn hydrate<const FROM_SERVER: bool>(self, cursor: &Cursor<Rndr>, position: &PositionState) -> Self::State {
				paste::paste! {
					let ([<$first:lower>], $([<$ty:lower>],)* ) = self;
					(
						[<$first:lower>].hydrate::<FROM_SERVER>(cursor, position),
						$([<$ty:lower>].hydrate::<FROM_SERVER>(cursor, position)),*
					)
				}
			}
		}

		impl<$first, $($ty),*> ToTemplate for ($first, $($ty,)*)
		where
			$first: ToTemplate,
			$($ty: ToTemplate),*
		{
			fn to_template(buf: &mut String, position: &mut Position)  {
				paste::paste! {
					$first ::to_template(buf, position);
					$($ty::to_template(buf, position));*;
				}
			}
		}

		impl<$first, $($ty),*, Rndr> Mountable<Rndr> for ($first, $($ty,)*) where
			$first: Mountable<Rndr>,
			$($ty: Mountable<Rndr>),*,
			Rndr: Renderer
		{
			fn unmount(&mut self) {
				paste::paste! {
					let ([<$first:lower>], $([<$ty:lower>],)*) = self;
					[<$first:lower>].unmount();
					$([<$ty:lower>].unmount());*
				}
			}

			fn mount(
				&mut self,
				parent: &Rndr::Element,
				marker: Option<&Rndr::Node>,
			) {
				paste::paste! {
					let ([<$first:lower>], $([<$ty:lower>],)*) = self;
					[<$first:lower>].mount(parent, marker);
					$([<$ty:lower>].mount(parent, marker));*
				}
			}

			fn insert_before_this(
				&self,
				parent: &Rndr::Element,
				child: &mut dyn Mountable<Rndr>,
			) -> bool {
				paste::paste! {
					let ([<$first:lower>], $([<$ty:lower>],)*) = self;
					[<$first:lower>].insert_before_this(parent, child)
					$(|| [<$ty:lower>].insert_before_this(parent, child))*
				}
			}
		}
	};
}

impl_view_for_tuples!(A, B);
impl_view_for_tuples!(A, B, C);
impl_view_for_tuples!(A, B, C, D);
impl_view_for_tuples!(A, B, C, D, E);
impl_view_for_tuples!(A, B, C, D, E, F);
impl_view_for_tuples!(A, B, C, D, E, F, G);
impl_view_for_tuples!(A, B, C, D, E, F, G, H);
impl_view_for_tuples!(A, B, C, D, E, F, G, H, I);
impl_view_for_tuples!(A, B, C, D, E, F, G, H, I, J);
impl_view_for_tuples!(A, B, C, D, E, F, G, H, I, J, K);
impl_view_for_tuples!(A, B, C, D, E, F, G, H, I, J, K, L);
impl_view_for_tuples!(A, B, C, D, E, F, G, H, I, J, K, L, M);
impl_view_for_tuples!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
impl_view_for_tuples!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
impl_view_for_tuples!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
impl_view_for_tuples!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q);
impl_view_for_tuples!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R);
impl_view_for_tuples!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S);
impl_view_for_tuples!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T
);
impl_view_for_tuples!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U
);
impl_view_for_tuples!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V
);
impl_view_for_tuples!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W
);
impl_view_for_tuples!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X
);
impl_view_for_tuples!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y
);
impl_view_for_tuples!(
    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y,
    Z
);
