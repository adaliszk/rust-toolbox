// SPDX-FileCopyrightText: 2026, Ádám Liszkai and Contributors
// SPDX-License-Identifier: MPL-2.0

use crate::Container;
use async_trait::async_trait;
use std::any::TypeId;

/// A boxed, thread-safe error returned by event listeners.
pub type AsyncError = Box<dyn std::error::Error + Send + Sync>;

/// The result an event listener returns: `Ok(())` on success, or any [`AsyncError`].
pub type EventResult = Result<(), AsyncError>;

/// A type that can be dispatched to its listeners.
///
/// The [`event`](crate::event) macro implements this for each marked struct.
pub trait Dispatchable: Clone + Send + Sync + 'static {}

#[doc(hidden)] // This is internal between the macro and the package
#[async_trait]
pub trait EventRegistration: Send + Sync
{
    fn event(&self) -> TypeId;
    fn subscribe(&'static self, bus: &mut crate::EventDispatcher);
    async fn resolve(&self, container: &Container) -> EventResult;
}

inventory::collect!(&'static dyn EventRegistration);
