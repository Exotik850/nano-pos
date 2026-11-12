use dioxus::prelude::*;

use crate::Route;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct SearchResult {
    pub route: Route,
    pub title: String,
    pub description: String,
}

#[server(_session: crate::auth::Session)]
pub async fn search_all(query: String) -> Result<Vec<SearchResult>, ServerFnError> {
    let mut conn = crate::server::db::establish_connection();
    crate::server::search::gather_search_results(&query, &mut conn)
        .map_err(|error| ServerFnError::Response(format!("Could not perform search: {error}")))
}
