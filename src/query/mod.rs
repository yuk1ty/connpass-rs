use self::types::OrderOption;

pub mod builder;
pub mod types;
pub mod validator;

/// A query data class to extract the specific data from connpass API.
/// For more details about respective fields: https://connpass.com/about/api/
/// The struct is along with the specification.
#[derive(PartialEq, Debug)]
pub struct Query {
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
    count: Option<u8>,
    format: Option<String>,
}

impl Default for Query {
    fn default() -> Self {
        Query {
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

impl Query {
    pub(crate) fn make_reqwest_query(self) -> Vec<(String, String)> {
        let mut queries = Vec::new();

        assemble_query_param(&mut queries, self.event_id, "event_id");
        assemble_query_param(&mut queries, self.keyword, "keyword");
        assemble_query_param(&mut queries, self.keyword_or, "keyword_or");
        assemble_query_param(&mut queries, self.ym, "ym");
        assemble_query_param(&mut queries, self.ymd, "ymd");
        assemble_query_param(&mut queries, self.nickname, "nickname");
        assemble_query_param(&mut queries, self.owner_nickname, "owner_nickname");
        assemble_query_param(&mut queries, self.series_id, "series_id");
        assemble_single_query_param(&mut queries, self.start, "start");
        assemble_single_query_param(&mut queries, self.order.map(|v| v.to_u8()), "order");
        assemble_single_query_param(&mut queries, self.count, "count");
        assemble_single_query_param(&mut queries, self.format, "format");

        queries
    }
}

fn assemble_query_param<T: ToString>(
    queries: &mut Vec<(String, String)>,
    source: Option<Vec<T>>,
    query_key: &str,
) {
    if let Some(elems) = source {
        for elem in elems {
            queries.push(make_elem(query_key, elem));
        }
    }
}

fn assemble_single_query_param<T: ToString>(
    queries: &mut Vec<(String, String)>,
    source: Option<T>,
    query_key: &str,
) {
    if let Some(elem) = source {
        queries.push(make_elem(query_key, elem));
    }
}

fn make_elem<T: ToString>(key: &str, value: T) -> (String, String) {
    (key.to_string(), value.to_string())
}

#[cfg(test)]
mod test {
    use crate::query::{assemble_query_param, assemble_single_query_param, make_elem};

    use super::{builder::QueryBuilder, types::OrderOption};

    #[test]
    fn test_assemble_query_param() {
        let mut queries = vec![("event_id".to_string(), "1".to_string())];
        assemble_query_param(&mut queries, Some(vec![2, 3]), "event_id");
        assert_eq!(queries.len(), 3);

        let mut queries = vec![("event_id".to_string(), "1".to_string())];
        assemble_query_param::<u8>(&mut queries, None, "event_id");
        assert_eq!(queries.len(), 1);
    }

    #[test]
    fn test_assemble_single_query_param() {
        let mut queries = vec![("event_id".to_string(), "1".to_string())];
        assemble_single_query_param(&mut queries, Some(2), "event_id");
        assert_eq!(queries.len(), 2);

        let mut queries = vec![("event_id".to_string(), "1".to_string())];
        assemble_single_query_param::<u8>(&mut queries, None, "event_id");
        assert_eq!(queries.len(), 1);
    }

    #[test]
    fn test_make_elem() {
        let (key, value) = make_elem("key", 1);
        assert_eq!(key, "key".to_string());
        assert_eq!(value, "1".to_string());
    }

    #[test]
    fn test_make_reqwest_query_partially() {
        let builder = QueryBuilder::begin()
            .event_ids(vec![1, 2, 3])
            .keyword("Rust".to_string())
            .start(1)
            .order(OrderOption::Newer)
            .count(10)
            .format("json".to_string());
        let query = builder.build().unwrap();
        let reqwest_query = query.make_reqwest_query();
        assert_eq!(
            reqwest_query,
            vec![
                make_elem("event_id", 1),
                make_elem("event_id", 2),
                make_elem("event_id", 3),
                make_elem("keyword", "Rust"),
                make_elem("start", 1),
                make_elem("order", OrderOption::Newer.to_u8()),
                make_elem("count", 10),
                make_elem("format", "json")
            ]
        );
    }
}
