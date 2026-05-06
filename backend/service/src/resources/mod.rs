//! Process-wide resource registry for singleton services.

mod error;

use std::{
    any::{Any, TypeId},
    sync::{Arc, LazyLock},
};

use dashmap::DashMap;
pub use error::ResourceNotRegisteredErr;

static REGISTRY: LazyLock<ResourceRegistry> = LazyLock::new(|| ResourceRegistry(DashMap::new()));

#[async_trait::async_trait]
pub trait Resource: Any + Send + Sync {
    async fn init() -> anyhow::Result<Self>
    where Self: Sized;
}

/// A type-keyed store that holds process-wide singleton resources.
///
/// Each resource type occupies exactly one slot, keyed by its [`TypeId`].
///
/// Call [`ResourceRegistry::register`] once per type during service startup,
/// then [`ResourceRegistry::get`] everywhere else.
#[derive(Clone)]
pub struct ResourceRegistry(DashMap<TypeId, Arc<dyn Any + Send + Sync>>);

impl ResourceRegistry {
    /// Registers a value of type [`Resource::init`] in the global registry, replacing any
    /// previously registered value of the same type.
    ///
    /// Must only be called during service startup, before any call to
    /// [`Self::get`] for the same type.
    pub async fn register<T: Resource>() -> anyhow::Result<Arc<T>> {
        let value = Arc::new(T::init().await?);
        Self::register_with(value.clone());
        Ok(value)
    }

    /// Registers a value of type `T` in the global registry, replacing any
    /// previously registered value of the same type.
    ///
    /// Must only be called during service startup, before any call to
    /// [`Self::get`] for the same type.
    pub fn register_with<T: Any + Send + Sync>(value: Arc<T>) {
        REGISTRY.0.insert(TypeId::of::<T>(), value);
    }

    /// Returns a `'static` reference to the registered value of type `T`.
    ///
    /// # Errors
    /// - [`ResourceNotRegisteredErr`]
    pub fn get<T: Any + Send + Sync>() -> Result<Arc<T>, ResourceNotRegisteredErr<T>> {
        REGISTRY
            .0
            .get(&TypeId::of::<T>())
            .ok_or(ResourceNotRegisteredErr::default())
            .and_then(|v| {
                v.clone()
                    .downcast::<T>()
                    .map_err(|_| ResourceNotRegisteredErr::default())
            })
    }
}

#[cfg(test)]
mod tests {
    use std::{fmt::Debug, sync::Arc};

    use test_case::test_case;

    use super::*;

    #[test_case(25_u32)]
    #[test_case(25_u64)]
    #[test_case(true)]
    #[allow(clippy::needless_pass_by_value)]
    fn resources_test<T: Debug + Clone + PartialEq + Eq + Send + Sync + 'static>(v: T) {
        ResourceRegistry::register_with(Arc::new(v.clone()));
        assert_eq!(*ResourceRegistry::get::<T>().unwrap(), v);
        assert!(ResourceRegistry::get::<()>().is_err());
    }
}
