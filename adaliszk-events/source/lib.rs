// SPDX-FileCopyrightText: 2026, Ádám Liszkai and Contributors
// SPDX-License-Identifier: MPL-2.0
//!
//! A [`tokio::sync::broadcast`]-backed event bus over
//! [`inventory`](https://crates.io/crates/inventory)-collected listeners, wired through the
//! [`adaliszk-anymap`](adaliszk_anymap) dependency-injection [`Container`]. An event is a plain
//! struct marked with [`event`]; a listener is a free `async fn` marked with [`listen`] bound to
//! its event with `event = TheEvent`, and every parameter is pulled from the container by
//! type (in any position) when an emitted event reaches it.
//!
//! The `Resolvable` derive expanded by [`event`] resolves to
//! [`adaliszk-anymap`](https://crates.io/crates/adaliszk-anymap) paths, so it is expected as a
//! direct dependency of the application alongside this crate.
//!

/// Re-export of [`adaliszk-anymap`](https://crates.io/crates/adaliszk-anymap)'s container.
pub use adaliszk_anymap::Container;

/// Mark a struct as a dispatchable event; see the macro's docs.
pub use adaliszk_events_macros::event;

/// Register a listener for an event from a free `async fn`; see the macro's docs.
pub use adaliszk_events_macros::listen;

#[doc(hidden)] // Used by the macro-generated registrations only
pub use async_trait::async_trait;

mod events;
pub use events::*;

#[doc(hidden)] // Used by the macro-generated registrations only
pub use inventory;

mod prelude;
pub use prelude::*;
