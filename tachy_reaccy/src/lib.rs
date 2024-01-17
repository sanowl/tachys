// The `arena` module appears to be internal and might be related to memory management or object pooling.
mod arena;

// Public modules for asynchronous signaling, context management, effects, memoization, etc.
pub mod async_signal;
pub mod context;
pub mod effect;
pub mod memo;

// The `notify` module is internal, possibly used for internal event notification.
mod notify;

pub mod render_effect;
pub mod selector;

// Conditional compilation for `serde` support.
#[cfg(feature = "serde")]
mod serde;

pub mod serialization;
pub mod shared_context;
pub mod signal;
pub mod signal_traits;

// Source module seems to be internal, possibly related to data sources or event sources.
mod source;

pub mod spawn;
pub mod store;

// Using specific items from the `source` and `arena` modules.
use crate::source::AnySubscriber;
pub use arena::{Owner, Root};
// Utilizing futures for asynchronous programming.
use futures::{Future, Stream};
use std::{cell::RefCell, pin::Pin};

// A prelude module to provide easy access to commonly used items.
pub mod prelude {
    pub use crate::{
        async_signal::{AsyncDerived, Resource},
        context::{provide_context, use_context},
        effect::Effect,
        memo::{ArcMemo, Memo},
        signal::{signal, ArcRwSignal, ReadSignal, RwSignal},
        signal_traits::*,
        store::{StoreField, StoreFieldIndex, StoreFieldIterator},
        Root,
    };
}

// Thread-local storage for an observer, used in reactive programming patterns.
thread_local! {
    static OBSERVER: RefCell<Option<AnySubscriber>> = RefCell::new(None);
}

// Type aliases for pinned futures and streams, enhancing code readability.
pub type PinnedFuture<T> = Pin<Box<dyn Future<Output = T> + Send + Sync>>;
pub type PinnedStream<T> = Pin<Box<dyn Stream<Item = T> + Send + Sync>>;

// Implementation of the Observer pattern for reactive programming.
pub(crate) struct Observer {}

impl Observer {
    fn get() -> Option<AnySubscriber> {
        OBSERVER.with(|o| o.borrow().clone())
    }

    fn is(observer: &AnySubscriber) -> bool {
        OBSERVER.with(|o| o.borrow().as_ref() == Some(observer))
    }

    fn take() -> Option<AnySubscriber> {
        OBSERVER.with(|o| o.borrow_mut().take())
    }

    fn set(observer: Option<AnySubscriber>) {
        OBSERVER.with(|o| *o.borrow_mut() = observer);
    }
}

// Function to execute a closure without tracking dependencies in a reactive system.
pub fn untrack<T>(fun: impl FnOnce() -> T) -> T {
    let prev = Observer::take();
    let value = fun();
    Observer::set(prev);
    value
}

// Logging functions with conditional compilation for web and non-web environments.
#[cfg(feature = "web")]
pub fn log(s: &str) {
    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(s));
}

#[cfg(not(feature = "web"))]
pub fn log(s: &str) {
    println!("{s}");
}
