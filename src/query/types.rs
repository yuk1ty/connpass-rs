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

impl Into<u8> for OrderOption {
    fn into(self) -> u8 {
        match self {
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
