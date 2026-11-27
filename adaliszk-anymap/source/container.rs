// SPDX-FileCopyrightText: 2026, Ádám Liszkai and Contributors
// SPDX-License-Identifier: MPL-2.0

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::OnceLock;

/// Builds a type-erased value from the container on demand.
type Factory = Box<dyn Fn(&Container) -> Box<dyn Any + Send + Sync> + Send + Sync>;

/// Type-keyed container holding at most one provider per type.
///
/// Provider is either a bound value (via [`bind`](Container::bind)) or a factory closure
/// (via [`bind_with`](Container::bind_with)). Factories are Laravel-style: they run on every
/// resolve and can pull their own dependencies out of the container.
pub struct Container
{
    instances: anymap3::Map<dyn Any + Send + Sync>,
    factories: HashMap<TypeId, Factory>,
}

impl Container
{
    /// Bind a ready value, replacing any previous value of the same type. Chainable.
    pub fn bind<T: Any + Send + Sync>(&mut self, value: T) -> &mut Self
    {
        self.instances.insert(value);
        self
    }

    /// Bind a factory that builds `T` on each resolve, replacing any previous factory.
    ///
    /// The closure receives the container, so it can resolve its own dependencies. A
    /// factory takes precedence over a bound value of the same type.
    ///
    /// Chainable.
    pub fn bind_with<T, F>(&mut self, factory: F) -> &mut Self
    where
        T: Any + Send + Sync,
        F: Fn(&Container) -> T + Send + Sync + 'static,
    {
        self.factories.insert(
            TypeId::of::<T>(),
            Box::new(move |container| Box::new(factory(container))),
        );
        self
    }

    /// Bind a factory that builds `T` once, lazily, then caches it for every later resolve.
    ///
    /// The factory runs on the first resolve and never again; subsequent resolves clone the
    /// cached value (so `T` must be `Clone`). For a shared instance, make `T` an `Arc<_>`:
    /// the cached `Arc` is cloned, so every resolve sees the same underlying value.
    ///
    /// Chainable.
    pub fn singleton_with<T, F>(&mut self, factory: F) -> &mut Self
    where
        T: Any + Send + Sync + Clone,
        F: Fn(&Container) -> T + Send + Sync + 'static,
    {
        let cache: OnceLock<T> = OnceLock::new();
        self.factories.insert(
            TypeId::of::<T>(),
            Box::new(move |container| Box::new(cache.get_or_init(|| factory(container)).clone())),
        );
        self
    }

    /// Whether a value or factory is bound for type `T`.
    pub fn contains<T: Any + Send + Sync>(&self) -> bool
    {
        self.instances.get::<T>().is_some() || self.factories.contains_key(&TypeId::of::<T>())
    }

    /// Build an owned `T` from its factory, if one is bound. Returns `None` otherwise.
    ///
    /// Unlike cloning a bound value, this needs no `Clone` bound, so it works for foreign
    /// types the application wires up with [`bind_with`](Container::bind_with).
    pub fn make<T: Any + Send + Sync>(&self) -> Option<T>
    {
        let factory = self.factories.get(&TypeId::of::<T>())?;
        factory(self).downcast::<T>().ok().map(|value| *value)
    }

    /// Borrow the bound value of type `T`, if one was bound with [`bind`](Container::bind).
    ///
    /// This never runs a factory; use [`make`](Container::make) or
    /// [`resolve`](Container::resolve) for factory-built instances.
    pub fn get<T: Any + Send + Sync>(&self) -> Option<&T>
    {
        self.instances.get::<T>()
    }

    /// Resolve a [`Resolvable`] type out of this container.
    pub fn resolve<T: Resolvable>(&self) -> Option<T>
    {
        T::resolve(self)
    }
}

impl Default for Container
{
    fn default() -> Self
    {
        Self { instances: anymap3::Map::new(), factories: HashMap::new() }
    }
}

/// A type that can build itself from a [`Container`].
///
/// Derive it with `#[derive(Resolvable)]` to prefer the type's factory and fall back to its
/// bound value (so the type must be `Clone`); implement it by hand for custom resolution.
pub trait Resolvable: Sized
{
    /// Resolve `Self` from `container`, or `None` when it cannot be produced.
    fn resolve(container: &Container) -> Option<Self>;
}
