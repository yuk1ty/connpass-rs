use std::convert::TryFrom;

use crate::errors::{ConnpassCliError, ConnpassResult, ValidationError};

use super::validator::Validator;

pub(crate) struct FetchCountRange(pub u8);

impl Validator for FetchCountRange {
    fn validate(self) -> ConnpassResult<Self> {
        match self.0 {
            1..=100 => Ok(self),
            _ => Err(ConnpassCliError::Validation(ValidationError::OutOfRange {
                msg: "`count` should be greater than or equal to 1 or less than or equals to 100. See more details: https://connpass.com/about/api/"
                    .to_string(),
            })),
        }
    }
}

pub(crate) struct FormatJson(pub String);

impl Validator for FormatJson {
    fn validate(self) -> ConnpassResult<Self> {
        if self.0 == "json" {
            Ok(self)
        } else {
            Err(ConnpassCliError::Validation(ValidationError::InvalidToken {
                msg: "`format` can just accept the string \"json\". See more details: https://connpass.com/about/api/".to_string(),
            }))
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum OrderOption {
    LastModifiedDate = 1,
    EventDate = 2,
    Newer = 3,
}

impl OrderOption {
    pub fn new(id: u8) -> ConnpassResult<Self> {
        Self::try_from(id)
    }

    pub fn to_u8(self) -> u8 {
        self.into()
    }
}

impl From<OrderOption> for u8 {
    fn from(opt: OrderOption) -> Self {
        match opt {
            OrderOption::LastModifiedDate => 1,
            OrderOption::EventDate => 2,
            OrderOption::Newer => 3,
        }
    }
}

impl TryFrom<u8> for OrderOption {
    type Error = ConnpassCliError;

    fn try_from(value: u8) -> ConnpassResult<Self> {
        match value {
            1 => Ok(OrderOption::LastModifiedDate),
            2 => Ok(OrderOption::EventDate),
            3 => Ok(OrderOption::Newer),
            _ => Err(ConnpassCliError::Validation(
                ValidationError::InvalidToken {
                    msg: format!("Invalid id came here: {}", &value),
                },
            )),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        errors::{ConnpassCliError, ValidationError},
        query::validator::Validator,
    };

    use super::{FetchCountRange, FormatJson};

    #[test]
    fn test_validate_fetch_count_range() {
        let value = FetchCountRange(1);
        let r = value.validate();
        assert!(r.is_ok());

        let value = FetchCountRange(100);
        let r = value.validate();
        assert!(r.is_ok());

        let value = FetchCountRange(0);
        let r = value.validate();
        assert!(matches!(
            r,
            Err(ConnpassCliError::Validation(ValidationError::OutOfRange {
                msg: _
            }))
        ));

        let value = FetchCountRange(101);
        let r = value.validate();
        assert!(matches!(
            r,
            Err(ConnpassCliError::Validation(ValidationError::OutOfRange {
                msg: _
            }))
        ));
    }

    #[test]
    fn test_validate_format_token() {
        let value = FormatJson("json".to_string());
        let r = value.validate();
        assert!(r.is_ok());

        let value = FormatJson("yaml".to_string());
        let r = value.validate();
        assert!(matches!(
            r,
            Err(ConnpassCliError::Validation(
                ValidationError::InvalidToken { msg: _ }
            ))
        ));
    }
}
