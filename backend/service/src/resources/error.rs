use std::{fmt::Debug, marker::PhantomData};

/// Returned by [`super::ResourceRegistry::get`] when a type has not been
/// registered.
#[derive(thiserror::Error)]
#[error("{} has not been registered in the resource registry", std::any::type_name::<T>())]
pub struct ResourceNotRegisteredErr<T>(PhantomData<T>);

impl<T> Default for ResourceNotRegisteredErr<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<T> Debug for ResourceNotRegisteredErr<T> {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            f,
            "{} has not been registered in the resource registry",
            std::any::type_name::<T>()
        )
    }
}
