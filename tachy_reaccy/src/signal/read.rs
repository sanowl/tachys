use super::ArcRwSignal;
use crate::{
    arena::{Stored, StoredData},
    prelude::{DefinedAt, SignalWithUntracked},
    source::{AnySubscriber, ReactiveNode, Source},
};
use std::panic::Location;

pub struct ReadSignal<T: Send + Sync + 'static> {
    pub(crate) inner: Stored<ArcReadSignal<T>>,
}

impl<T: Send + Sync + 'static> StoredData for ReadSignal<T> {
    type Data = ArcReadSignal<T>;

    fn get(&self) -> Option<Self::Data> {
        self.inner.get()
    }

    fn dispose(&self) {
        self.inner.dispose();
    }
}

pub struct ArcReadSignal<T>(pub(crate) ArcRwSignal<T>);

impl<T> Clone for ArcReadSignal<T> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<T> ReactiveNode for ArcReadSignal<T> {
    fn mark_dirty(&self) {
        self.0.mark_dirty();
    }

    fn mark_check(&self) {
        self.0.mark_check();
    }

    fn mark_subscribers_check(&self) {
        self.0.mark_subscribers_check();
    }

    fn update_if_necessary(&self) -> bool {
        self.0.update_if_necessary()
    }
}

impl<T> Source for ArcReadSignal<T> {
    fn add_subscriber(&self, subscriber: AnySubscriber) {
        self.0.add_subscriber(subscriber);
    }

    fn remove_subscriber(&self, subscriber: &AnySubscriber) {
        self.0.remove_subscriber(subscriber);
    }

    fn clear_subscribers(&self) {
        self.0.clear_subscribers();
    }
}

impl<T> DefinedAt for ArcReadSignal<T> {
    fn defined_at(&self) -> Option<&'static Location<'static>> {
        self.0.defined_at()
    }
}

impl<T> SignalWithUntracked for ArcReadSignal<T> {
    type Value = T;

    fn try_with_untracked<U>(
        &self,
        fun: impl FnOnce(&Self::Value) -> U,
    ) -> Option<U> {
        self.0.try_with_untracked(fun)
    }
}