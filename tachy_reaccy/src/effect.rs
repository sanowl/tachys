use crate::{
    arena::Owner,
    notify::NotificationSender,
    source::{
        AnySource, AnySubscriber, ReactiveNode, ReactiveNodeState, SourceSet,
        Subscriber, ToAnySubscriber,
    },
    spawn::spawn,
};
use futures::StreamExt;
use parking_lot::RwLock;
use std::{
    mem,
    sync::{Arc, Weak},
};

pub struct Effect<T>
where
    T: 'static,
{
    value: Arc<RwLock<Option<T>>>,
    inner: Arc<RwLock<EffectInner>>,
}

pub(crate) struct EffectInner {
    pub observer: NotificationSender,
    pub sources: SourceSet,
}

impl<T> Clone for Effect<T> {
    fn clone(&self) -> Self {
        Self {
            value: Arc::clone(&self.value),
            inner: Arc::clone(&self.inner),
        }
    }
}

impl<T> Effect<T>
where
    T: Send + Sync + 'static,
{
    pub fn new(
        mut fun: impl FnMut(Option<T>) -> T + Send + Sync + 'static,
    ) -> Self {
        let (mut observer, mut rx) = NotificationSender::channel();

        // spawn the effect asynchronously
        // we'll notify once so it runs on the next tick,
        // to register observed values
        observer.notify();

        let value = Arc::new(RwLock::new(None));
        let inner = Arc::new(RwLock::new(EffectInner {
            observer,
            sources: SourceSet::new(),
        }));
        let owner = Owner::new();

        spawn({
            let value = Arc::clone(&value);
            let subscriber = inner.to_any_subscriber();

            async move {
                while rx.next().await.is_some() {
                    subscriber.clear_sources(&subscriber);

                    let old_value = mem::take(&mut *value.write());
                    let new_value = owner
                        .with(|| subscriber.with_observer(|| fun(old_value)));
                    *value.write() = Some(new_value);
                }
            }
        });
        Self { value, inner }
    }

    pub fn with_value_mut<U>(
        &self,
        fun: impl FnOnce(&mut T) -> U,
    ) -> Option<U> {
        self.value.write().as_mut().map(fun)
    }
}

impl<T> ToAnySubscriber for Effect<T> {
    fn to_any_subscriber(&self) -> AnySubscriber {
        self.inner.to_any_subscriber()
    }
}

impl ToAnySubscriber for Arc<RwLock<EffectInner>> {
    fn to_any_subscriber(&self) -> AnySubscriber {
        AnySubscriber(
            self.data_ptr() as usize,
            Arc::downgrade(self) as Weak<dyn Subscriber + Send + Sync>,
        )
    }
}

impl ReactiveNode for RwLock<EffectInner> {
    fn set_state(&self, _state: ReactiveNodeState) {}

    fn mark_subscribers_check(&self) {}

    fn update_if_necessary(&self) -> bool {
        let mut lock = self.write();
        for source in lock.sources.take() {
            if source.update_if_necessary() {
                lock.observer.notify();
                return true;
            }
        }
        false
    }

    fn mark_check(&self) {
        self.write().observer.notify()
    }

    fn mark_dirty(&self) {
        self.write().observer.notify()
    }
}

impl Subscriber for RwLock<EffectInner> {
    fn add_source(&self, source: AnySource) {
        self.write().sources.insert(source);
    }

    fn clear_sources(&self, subscriber: &AnySubscriber) {
        self.write().sources.clear_sources(subscriber);
    }
}
