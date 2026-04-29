/// A helper trait mirroring [`TryInto`] with a unified error type.
pub trait ConvertTo<T> {
    fn convert(self) -> Result<T, ConvertError>;
}

impl<FromT, IntoT> ConvertTo<IntoT> for FromT
where
    FromT: TryInto<IntoT>,
    FromT::Error: Into<anyhow::Error> + 'static,
{
    fn convert(self) -> Result<IntoT, ConvertError> {
        self.try_into()
            .map_err(|e| ConvertError::new::<FromT, IntoT>(e))
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Cannot convert type '{t1}' to type '{t2}', err: {err}")]
pub struct ConvertError {
    t1: &'static str,
    t2: &'static str,
    err: anyhow::Error,
}

impl ConvertError {
    pub fn new<FromT, IntoT>(err: FromT::Error) -> Self
    where
        FromT: TryInto<IntoT>,
        FromT::Error: Into<anyhow::Error> + 'static,
    {
        Self {
            t1: std::any::type_name::<FromT>(),
            t2: std::any::type_name::<IntoT>(),
            err: err.into(),
        }
    }
}
