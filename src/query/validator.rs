use super::error::ValidationError;

pub(crate) trait Validator
where
    Self: Sized,
{
    fn validate(self) -> Result<Self, ValidationError>;
}
