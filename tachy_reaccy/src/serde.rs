use crate::{signal::Signal, signal_traits::SignalWith};
use serde::{Deserialize, Serialize};

impl<T: Send + Sync + Serialize> Serialize for Signal<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.with(|value| value.serialize(serializer))
    }
}

impl<'de, T: Send + Sync + Deserialize<'de>> Deserialize<'de> for Signal<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        T::deserialize(deserializer).map(Signal::new)
    }
}
