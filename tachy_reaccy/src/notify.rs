use crate::{
    arena::Owner,
    source::{ReactiveNodeState, Subscriber},
    Queue, OBSERVER,
};
//use browser_only_send::BrowserOnly;
use futures::channel::mpsc::{channel, Receiver, Sender};
use parking_lot::RwLock;
use rustc_hash::FxHashSet;
use std::{
    collections::hash_set::IntoIter,
    fmt::Debug,
    hash::Hash,
    mem,
    ops::Deref,
    ptr,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

#[derive(Debug, Clone)]
pub struct NotificationSender(Sender<()>);

impl NotificationSender {
    pub fn notify(&mut self) {
        // if this fails, it's because there's already a message
        // in the buffer. but we're just sending () to wake it up;
        // we really don't care if multiple signals try to notify it synchronously
        // and it fails to send, as long as it's sent the one time
        _ = self.0.try_send(());
    }
}

impl Hash for NotificationSender {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash_receiver(state);
    }
}

impl PartialEq for NotificationSender {
    fn eq(&self, other: &Self) -> bool {
        self.0.same_receiver(&other.0)
    }
}

#[derive(Clone)]
pub struct EffectNotifier {
    pub(crate) tx: Arc<RwLock<NotificationSender>>,
    removers: Queue<Box<dyn FnOnce() + Send + Sync>>,
}

impl Hash for EffectNotifier {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        ptr::hash(&self.tx, state)
    }
}

impl PartialEq for EffectNotifier {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.tx, &other.tx)
    }
}

impl Eq for EffectNotifier {}

impl EffectNotifier {
    pub fn new() -> (Self, Receiver<()>) {
        let (tx, rx) = channel::<()>(1);
        (
            Self {
                tx: Arc::new(RwLock::new(NotificationSender(tx))),
                removers: Default::default(),
            },
            rx,
        )
    }

    pub fn notify(&self) {
        self.tx.write().notify();
    }

    pub fn cleanup(&self) {
        for remover in mem::take(&mut *self.removers.write()) {
            remover();
        }
    }
}