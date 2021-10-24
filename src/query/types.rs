use super::{error::ValidationError, validator::Validator};

pub(crate) struct FetchCountRange(pub u8);

impl Validator for FetchCountRange {
    fn validate(self) -> Result<Self, ValidationError> {
        match self.0 {
            1..=100 => Ok(self),
            _ => Err(ValidationError::OutOfRange {
                msg: "count (取得件数) は最小値1、最大値100です。".to_string(),
            }),
        }
    }
}

pub(crate) struct FormatJson(pub String);

impl Validator for FormatJson {
    fn validate(self) -> Result<Self, ValidationError> {
        if self.0 == "json" {
            Ok(self)
        } else {
            Err(ValidationError::InvalidToken {
                msg: "json という文字列のみ入力可能です。".to_string(),
            })
        }
    }
}

pub enum OrderOption {
    LastModifiedDate = 1,
    EventDate = 2,
    Newer = 3,
}

impl OrderOption {
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
