use crate::api::search::{SearchResult, SearchResults};
use diesel::prelude::*;
use dioxus::server::inventory;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub struct SearchFilter {
    query: fn(&str, &mut diesel::PgConnection) -> diesel::QueryResult<Vec<SearchResult>>,
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
) -> diesel::QueryResult<SearchResults> {
    let mut unnamed = Vec::new();
    let mut named = HashMap::new();
    conn.transaction(|conn| {
        for filter in inventory::iter::<SearchFilter> {
            let filter_results = (filter.query)(query, conn)?;
            for result in filter_results {
                if let Some(category) = result.category.as_ref() {
                    named
                        .entry(category.clone())
                        .or_insert_with(Vec::new)
                        .push(result);
                } else {
                    unnamed.push(result);
                }
            }
        }
        Ok::<_, diesel::result::Error>(())
    })?;
    Ok(SearchResults {
        named,
        unnamed,
    })
}
