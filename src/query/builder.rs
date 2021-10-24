use std::vec;

use super::{
    error::ValidationError,
    types::{FetchCountRange, FormatJson},
    validator::Validator,
    OrderOption, Query,
};

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

impl QueryBuilder {
    pub fn begin() -> Self {
        QueryBuilder::default()
    }

    pub fn event_ids(mut self, ids: Vec<u32>) -> Self {
        self.event_id = Some(ids);
        self
    }

    pub fn event_id(mut self, id: u32) -> Self {
        self.event_id = match self.event_id {
            Some(mut xs) => {
                xs.push(id);
                Some(xs)
            }
            None => Some(vec![id]),
        };
        self
    }

    pub fn keywords(mut self, keywords: Vec<String>) -> Self {
        self.keyword = Some(keywords);
        self
    }

    pub fn keyword(mut self, keyword: String) -> Self {
        self.keyword = match self.keyword {
            Some(mut xs) => {
                xs.push(keyword);
                Some(xs)
            }
            None => Some(vec![keyword]),
        };
        self
    }

    pub fn keywords_or(mut self, keywords: Vec<String>) -> Self {
        self.keyword_or = Some(keywords);
        self
    }

    pub fn keyword_or(mut self, keyword: String) -> Self {
        self.keyword_or = match self.keyword_or {
            Some(mut xs) => {
                xs.push(keyword);
                Some(xs)
            }
            None => Some(vec![keyword]),
        };
        self
    }

    pub fn yms(mut self, ym: Vec<u32>) -> Self {
        self.ym = Some(ym);
        self
    }

    pub fn ym(mut self, ym: u32) -> Self {
        self.ym = match self.ym {
            Some(mut xs) => {
                xs.push(ym);
                Some(xs)
            }
            None => Some(vec![ym]),
        };
        self
    }

    pub fn ymds(mut self, ymd: Vec<u32>) -> Self {
        self.ymd = Some(ymd);
        self
    }

    pub fn ymd(mut self, ymd: u32) -> Self {
        self.ymd = match self.ymd {
            Some(mut xs) => {
                xs.push(ymd);
                Some(xs)
            }
            None => Some(vec![ymd]),
        };
        self
    }

    pub fn nickname(mut self, nickname: String) -> Self {
        self.nickname = match self.nickname {
            Some(mut xs) => {
                xs.push(nickname);
                Some(xs)
            }
            None => Some(vec![nickname]),
        };
        self
    }

    pub fn nicknames(mut self, nickname: Vec<String>) -> Self {
        self.nickname = Some(nickname);
        self
    }

    pub fn owner_nicknames(mut self, owner_nickname: Vec<String>) -> Self {
        self.owner_nickname = Some(owner_nickname);
        self
    }

    pub fn owner_nickname(mut self, owner_nickname: String) -> Self {
        self.owner_nickname = match self.owner_nickname {
            Some(mut xs) => {
                xs.push(owner_nickname);
                Some(xs)
            }
            None => Some(vec![owner_nickname]),
        };
        self
    }

    pub fn series_ids(mut self, series_ids: Vec<u32>) -> Self {
        self.series_id = Some(series_ids);
        self
    }

    pub fn series_id(mut self, series_id: u32) -> Self {
        self.series_id = match self.series_id {
            Some(mut xs) => {
                xs.push(series_id);
                Some(xs)
            }
            None => Some(vec![series_id]),
        };
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

    pub fn build(self) -> Result<Query, ValidationError> {
        let mut query = Query::default();

        query.event_id = self.event_id;
        query.keyword = self.keyword;
        query.keyword_or = self.keyword_or;
        query.ym = self.ym;
        query.ymd = self.ymd;
        query.nickname = self.nickname;
        query.owner_nickname = self.owner_nickname;
        query.start = self.start;
        query.order = self.order;

        if let Some(count) = self.count {
            query.count = Some(count.validate()?.0);
        }

        if let Some(format) = self.format {
            query.format = Some(format.validate()?.0);
        }

        Ok(query)
    }
}
