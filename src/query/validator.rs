use crate::errors::ConnpassCliError;

pub(crate) trait Validator
where
    Self: Sized,
{
    fn validate(self) -> Result<Self, ConnpassCliError>;
}
