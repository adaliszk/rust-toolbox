// SPDX-FileCopyrightText: 2026, Ádám Liszkai and Contributors
// SPDX-License-Identifier: MPL-2.0

use adaliszk_anymap::{Container, Resolvable};
use std::sync::atomic::{AtomicU32, Ordering};

#[derive(Clone, PartialEq, Debug)]
struct Config
{
    token: String,
}

#[derive(Resolvable, Clone, PartialEq, Debug)]
struct Service
{
    id: u32,
}

#[test]
fn binds_and_reads_back_by_type()
{
    let mut container = Container::default();
    container.bind(Config { token: "abc".to_owned() });

    assert_eq!(container.borrow::<Config>().map(|c| c.token.as_str()), Some("abc"));
    assert!(container.contains::<Config>());
    assert!(!container.contains::<Service>());
}

#[test]
fn resolvable_derive_clones_bound_value_out()
{
    let mut container = Container::default();
    container.bind(Service { id: 7 });

    assert_eq!(container.require::<Service>().ok(), Some(Service { id: 7 }));
}

#[test]
fn missing_binding_resolves_to_none()
{
    let container = Container::default();

    assert_eq!(container.require::<Service>().ok(), None);
}

#[test]
fn require_names_the_dependency_in_the_error()
{
    let container = Container::default();

    let error = container.require::<Service>().unwrap_err();
    assert!(error.to_string().contains("could not resolve"));
    assert!(error.to_string().contains("Service"));
}

#[test]
fn bind_with_builds_the_value_from_a_factory()
{
    let mut container = Container::default();
    container.bind_with(|_| Service { id: 42 });

    assert_eq!(container.require::<Service>().ok(), Some(Service { id: 42 }));
    assert_eq!(container.make::<Service>(), Some(Service { id: 42 }));
}

#[test]
fn factory_runs_on_every_resolve()
{
    static CALLS: AtomicU32 = AtomicU32::new(0);

    let mut container = Container::default();
    container.bind_with(|_| Service { id: CALLS.fetch_add(1, Ordering::SeqCst) });

    assert_eq!(container.require::<Service>().ok(), Some(Service { id: 0 }));
    assert_eq!(container.require::<Service>().ok(), Some(Service { id: 1 }));
}

#[test]
fn factory_can_pull_its_own_dependencies_from_the_container()
{
    #[derive(Clone)]
    struct Db
    {
        url: String,
    }

    #[derive(Resolvable, Clone, PartialEq, Debug)]
    struct Repo
    {
        url: String,
    }

    let mut container = Container::default();
    container.bind(Db { url: "postgres://x".to_owned() });
    container.bind_with(|container| Repo {
        url: container.borrow::<Db>().unwrap().url.clone(),
    });

    assert_eq!(container.require::<Repo>().unwrap().url, "postgres://x");
}

#[test]
fn factory_takes_precedence_over_a_bound_value()
{
    let mut container = Container::default();
    container.bind(Service { id: 1 });
    container.bind_with(|_| Service { id: 2 });

    assert_eq!(container.require::<Service>().ok(), Some(Service { id: 2 }));
}

#[test]
fn singleton_with_builds_once_and_caches()
{
    static CALLS: AtomicU32 = AtomicU32::new(0);

    let mut container = Container::default();
    container.singleton_with(|_| {
        CALLS.fetch_add(1, Ordering::SeqCst);
        Service { id: 99 }
    });

    assert_eq!(container.require::<Service>().ok(), Some(Service { id: 99 }));
    assert_eq!(container.require::<Service>().ok(), Some(Service { id: 99 }));
    assert_eq!(CALLS.load(Ordering::SeqCst), 1);
}

#[test]
fn singleton_with_shares_one_instance_via_arc()
{
    use std::sync::Arc;

    let mut container = Container::default();
    container.singleton_with(|_| Arc::new(Service { id: 5 }));

    let first = container.make::<Arc<Service>>().unwrap();
    let second = container.make::<Arc<Service>>().unwrap();

    assert!(Arc::ptr_eq(&first, &second));
}
