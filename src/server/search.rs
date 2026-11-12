use crate::api::search::SearchResult;
use diesel::prelude::*;
use dioxus::server::inventory;

#[derive(Debug, Clone, Copy)]
pub struct SearchFilter {
    pub query: fn(&str, &mut diesel::PgConnection) -> diesel::QueryResult<Vec<SearchResult>>,
}
impl SearchFilter {
    pub const fn new(
        query: fn(&str, &mut diesel::PgConnection) -> diesel::QueryResult<Vec<SearchResult>>,
    ) -> Self {
        Self { query }
    }
}

inventory::collect!(SearchFilter);

pub fn gather_search_results(
    query: &str,
    conn: &mut diesel::PgConnection,
) -> diesel::QueryResult<Vec<SearchResult>> {
    let mut results = Vec::new();
    conn.transaction(|conn| {
        for filter in inventory::iter::<SearchFilter> {
            let filter_results = (filter.query)(query, conn)?;
            results.extend(filter_results);
        }
        Ok::<_, diesel::result::Error>(())
    })?;
    Ok(results)
}
