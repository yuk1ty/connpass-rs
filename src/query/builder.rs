use helper::*;

use crate::errors::ConnpassCliError;

use super::{
    types::{FetchCountRange, FormatJson},
    validator::Validator,
    OrderOption, Query,
};

/// Utility builder for building `query::Query`.
pub struct QueryBuilder {
    event_id: Option<Vec<u32>>,
    keyword: Option<Vec<String>>,
    keyword_or: Option<Vec<String>>,
    ym: Option<Vec<u32>>,
    ymd: Option<Vec<u32>>,
    nickname: Option<Vec<String>>,
    owner_nickname: Option<Vec<String>>,
    series_id: Option<Vec<u32>>,
    start: Option<u32>,
    order: Option<OrderOption>,
    count: Option<FetchCountRange>,
    format: Option<FormatJson>,
}

impl Default for QueryBuilder {
    fn default() -> Self {
        Self {
            event_id: None,
            keyword: None,
            keyword_or: None,
            ym: None,
            ymd: None,
            nickname: None,
            owner_nickname: None,
            series_id: None,
            start: None,
            order: None,
            count: None,
            format: None,
        }
    }
}

/// An implementation for QueryBuilder.
/// There are two function types:
/// 1. functions that can accept a single argument.
/// 2. functions that can acceps a `Vec` type argument.
///
/// The former ones simply add the accepted value to this builder,
/// but the latter ones always _replace_ the value in placed in this builder by the accepted one.
impl QueryBuilder {
    /// Initializes `QueryBuilder`.
    pub fn begin() -> Self {
        QueryBuilder::default()
    }

    pub fn event_ids(mut self, ids: Vec<u32>) -> Self {
        self.event_id = Some(ids);
        self
    }

    pub fn event_id(mut self, id: u32) -> Self {
        self.event_id = push_or_create(self.event_id, id);
        self
    }

    pub fn keywords(mut self, keywords: Vec<String>) -> Self {
        self.keyword = Some(keywords);
        self
    }

    pub fn keyword(mut self, keyword: String) -> Self {
        self.keyword = push_or_create(self.keyword, keyword);
        self
    }

    pub fn keywords_or(mut self, keywords: Vec<String>) -> Self {
        self.keyword_or = Some(keywords);
        self
    }

    pub fn keyword_or(mut self, keyword: String) -> Self {
        self.keyword_or = push_or_create(self.keyword_or, keyword);
        self
    }

    pub fn yms(mut self, ym: Vec<u32>) -> Self {
        self.ym = Some(ym);
        self
    }

    pub fn ym(mut self, ym: u32) -> Self {
        self.ym = push_or_create(self.ym, ym);
        self
    }

    pub fn ymds(mut self, ymd: Vec<u32>) -> Self {
        self.ymd = Some(ymd);
        self
    }

    pub fn ymd(mut self, ymd: u32) -> Self {
        self.ymd = push_or_create(self.ymd, ymd);
        self
    }

    pub fn nicknames(mut self, nickname: Vec<String>) -> Self {
        self.nickname = Some(nickname);
        self
    }

    pub fn nickname(mut self, nickname: String) -> Self {
        self.nickname = push_or_create(self.nickname, nickname);
        self
    }

    pub fn owner_nicknames(mut self, owner_nickname: Vec<String>) -> Self {
        self.owner_nickname = Some(owner_nickname);
        self
    }

    pub fn owner_nickname(mut self, owner_nickname: String) -> Self {
        self.owner_nickname = push_or_create(self.owner_nickname, owner_nickname);
        self
    }

    pub fn series_ids(mut self, series_ids: Vec<u32>) -> Self {
        self.series_id = Some(series_ids);
        self
    }

    pub fn series_id(mut self, series_id: u32) -> Self {
        self.series_id = push_or_create(self.series_id, series_id);
        self
    }

    pub fn start(mut self, start: u32) -> Self {
        self.start = Some(start);
        self
    }

    pub fn order(mut self, order: OrderOption) -> Self {
        self.order = Some(order);
        self
    }

    pub fn count(mut self, count: u8) -> Self {
        self.count = Some(FetchCountRange(count));
        self
    }

    pub fn format(mut self, format: String) -> Self {
        self.format = Some(FormatJson(format));
        self
    }

    /// Convert from `QueryBuilder` to `Query` with some validation checks.
    /// The following checks run in this function:
    /// 1. validate the `count` value in range of 0 to 100.
    /// 2. validate if the `format` value is just "json".
    ///
    /// These validation specifications are described in connpass's documentation.
    /// Please have a look at https://connpass.com/about/api/.
    pub fn build(self) -> Result<Query, ConnpassCliError> {
        let mut query = Query {
            event_id: self.event_id,
            keyword: self.keyword,
            keyword_or: self.keyword_or,
            ym: self.ym,
            ymd: self.ymd,
            nickname: self.nickname,
            owner_nickname: self.owner_nickname,
            series_id: self.series_id,
            start: self.start,
            order: self.order,
            ..Default::default()
        };

        if let Some(count) = self.count {
            query.count = Some(count.validate()?.0);
        }

        if let Some(format) = self.format {
            query.format = Some(format.validate()?.0);
        }

        Ok(query)
    }
}

mod helper {
    pub fn push_or_create<T>(source: Option<Vec<T>>, pushed: T) -> Option<Vec<T>> {
        match source {
            Some(mut xs) => {
                xs.push(pushed);
                Some(xs)
            }
            None => Some(vec![pushed]),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        errors::{ConnpassCliError, ValidationError},
        query::{types::OrderOption, Query},
    };

    use super::QueryBuilder;

    #[test]
    fn test_add_event_ids() {
        let builder = QueryBuilder::begin().event_ids(vec![1, 2, 3]);
        assert_eq!(
            builder.build().unwrap(),
            Query {
                event_id: Some(vec![1, 2, 3]),
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_add_event_id() {
        let builder = QueryBuilder::begin().event_id(1);
        assert_eq!(
            builder.build().unwrap(),
            Query {
                event_id: Some(vec![1]),
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_call_multiple_time_event_id() {
        let builder = QueryBuilder::begin().event_id(1).event_id(2).event_id(3);
        assert_eq!(
            builder.build().unwrap(),
            Query {
                event_id: Some(vec![1, 2, 3]),
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_add_keywords() {
        let builder = QueryBuilder::begin().keywords(vec![
            "Python".to_string(),
            "Rust".to_string(),
            "Swift".to_string(),
        ]);
        assert_eq!(
            builder.build().unwrap(),
            Query {
                keyword: Some(vec![
                    "Python".to_string(),
                    "Rust".to_string(),
                    "Swift".to_string()
                ]),
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_add_keyword() {
        let builder = QueryBuilder::begin().keyword("Rust".to_string());
        assert_eq!(
            builder.build().unwrap(),
            Query {
                keyword: Some(vec!["Rust".to_string()]),
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_call_multiple_time_keyword() {
        let builder = QueryBuilder::begin()
            .keyword("Python".to_string())
            .keyword("Rust".to_string())
            .keyword("Swift".to_string());
        assert_eq!(
            builder.build().unwrap(),
            Query {
                keyword: Some(vec![
                    "Python".to_string(),
                    "Rust".to_string(),
                    "Swift".to_string()
                ]),
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_add_keywords_or() {
        let builder = QueryBuilder::begin().keywords_or(vec![
            "Python".to_string(),
            "Rust".to_string(),
            "Swift".to_string(),
        ]);
        assert_eq!(
            builder.build().unwrap(),
            Query {
                keyword_or: Some(vec![
                    "Python".to_string(),
                    "Rust".to_string(),
                    "Swift".to_string()
                ]),
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_add_keyword_or() {
        let builder = QueryBuilder::begin().keyword_or("Rust".to_string());
        assert_eq!(
            builder.build().unwrap(),
            Query {
                keyword_or: Some(vec!["Rust".to_string()]),
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_call_multiple_time_keyword_or() {
        let builder = QueryBuilder::begin()
            .keyword_or("Python".to_string())
            .keyword_or("Rust".to_string())
            .keyword_or("Swift".to_string());
        assert_eq!(
            builder.build().unwrap(),
            Query {
                keyword_or: Some(vec![
                    "Python".to_string(),
                    "Rust".to_string(),
                    "Swift".to_string()
                ]),
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_add_yms() {
        let builder = QueryBuilder::begin().yms(vec![202101, 202102, 202103]);
        assert_eq!(
            builder.build().unwrap(),
            Query {
                ym: Some(vec![202101, 202102, 202103]),
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_add_ym() {
        let builder = QueryBuilder::begin().ym(202101);
        assert_eq!(
            builder.build().unwrap(),
            Query {
                ym: Some(vec![202101]),
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_call_multiple_time_ym() {
        let builder = QueryBuilder::begin().ym(202101).ym(202102).ym(202103);
        assert_eq!(
            builder.build().unwrap(),
            Query {
                ym: Some(vec![202101, 202102, 202103]),
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_add_ymds() {
        let builder = QueryBuilder::begin().ymds(vec![20210101, 20210201, 20210301]);
        assert_eq!(
            builder.build().unwrap(),
            Query {
                ymd: Some(vec![20210101, 20210201, 20210301]),
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_add_ymd() {
        let builder = QueryBuilder::begin().ymd(20210101);
        assert_eq!(
            builder.build().unwrap(),
            Query {
                ymd: Some(vec![20210101]),
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_call_multiple_time_ymd() {
        let builder = QueryBuilder::begin()
            .ymd(20210101)
            .ymd(20210201)
            .ymd(20210301);
        assert_eq!(
            builder.build().unwrap(),
            Query {
                ymd: Some(vec![20210101, 20210201, 20210301]),
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_add_nicknames() {
        let builder = QueryBuilder::begin().nicknames(vec![
            "Harry".to_string(),
            "Ron".to_string(),
            "Hermione".to_string(),
        ]);
        assert_eq!(
            builder.build().unwrap(),
            Query {
                nickname: Some(vec![
                    "Harry".to_string(),
                    "Ron".to_string(),
                    "Hermione".to_string()
                ]),
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_add_nickname() {
        let builder = QueryBuilder::begin().nickname("Harry".to_string());
        assert_eq!(
            builder.build().unwrap(),
            Query {
                nickname: Some(vec!["Harry".to_string()]),
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_call_multiple_time_nickname() {
        let builder = QueryBuilder::begin()
            .nickname("Harry".to_string())
            .nickname("Ron".to_string())
            .nickname("Hermione".to_string());
        assert_eq!(
            builder.build().unwrap(),
            Query {
                nickname: Some(vec![
                    "Harry".to_string(),
                    "Ron".to_string(),
                    "Hermione".to_string()
                ]),
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_add_owner_nicknames() {
        let builder = QueryBuilder::begin().owner_nicknames(vec![
            "Harry".to_string(),
            "Ron".to_string(),
            "Hermione".to_string(),
        ]);
        assert_eq!(
            builder.build().unwrap(),
            Query {
                owner_nickname: Some(vec![
                    "Harry".to_string(),
                    "Ron".to_string(),
                    "Hermione".to_string()
                ]),
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_add_owner_nickname() {
        let builder = QueryBuilder::begin().owner_nickname("Harry".to_string());
        assert_eq!(
            builder.build().unwrap(),
            Query {
                owner_nickname: Some(vec!["Harry".to_string()]),
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_call_multiple_time_owner_nickname() {
        let builder = QueryBuilder::begin()
            .owner_nickname("Harry".to_string())
            .owner_nickname("Ron".to_string())
            .owner_nickname("Hermione".to_string());
        assert_eq!(
            builder.build().unwrap(),
            Query {
                owner_nickname: Some(vec![
                    "Harry".to_string(),
                    "Ron".to_string(),
                    "Hermione".to_string()
                ]),
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_add_series_ids() {
        let builder = QueryBuilder::begin().series_ids(vec![1, 2, 3]);
        assert_eq!(
            builder.build().unwrap(),
            Query {
                series_id: Some(vec![1, 2, 3]),
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_add_series_id() {
        let builder = QueryBuilder::begin().series_id(1);
        assert_eq!(
            builder.build().unwrap(),
            Query {
                series_id: Some(vec![1]),
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_call_multiple_time_series_id() {
        let builder = QueryBuilder::begin().series_id(1).series_id(2).series_id(3);
        assert_eq!(
            builder.build().unwrap(),
            Query {
                series_id: Some(vec![1, 2, 3]),
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_call_start() {
        let builder = QueryBuilder::begin().start(1);
        assert_eq!(
            builder.build().unwrap(),
            Query {
                start: Some(1),
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_call_order() {
        let builder = QueryBuilder::begin().order(OrderOption::LastModifiedDate);
        assert_eq!(
            builder.build().unwrap(),
            Query {
                order: Some(OrderOption::LastModifiedDate),
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_call_count() {
        let builder = QueryBuilder::begin().count(50);
        assert_eq!(
            builder.build().unwrap(),
            Query {
                count: Some(50),
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_validation_count_range() {
        let builder = QueryBuilder::begin().count(0);
        assert!(matches!(
            builder.build(),
            Err(ConnpassCliError::Validation(ValidationError::OutOfRange {
                msg: _
            }))
        ));

        let builder = QueryBuilder::begin().count(101);
        assert!(matches!(
            builder.build(),
            Err(ConnpassCliError::Validation(ValidationError::OutOfRange {
                msg: _
            }))
        ));
    }

    #[test]
    fn test_call_format() {
        let builder = QueryBuilder::begin().format("json".to_string());
        assert_eq!(
            builder.build().unwrap(),
            Query {
                format: Some("json".to_string()),
                ..Default::default()
            }
        );
    }

    #[test]
    fn test_validation_format() {
        let builder = QueryBuilder::begin().format("yaml".to_string());
        assert!(matches!(
            builder.build(),
            Err(ConnpassCliError::Validation(
                ValidationError::InvalidToken { msg: _ }
            ))
        ));
    }
}
