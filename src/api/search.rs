use dioxus::prelude::*;

use serde::{Deserialize, Serialize};
use crate::Route;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct SearchResult {
    pub route: Route,
    pub title: String,
    pub description: String,
}

#[cfg(feature = "server")]
fn require_search_session(headers: &dioxus::fullstack::HeaderMap) -> Result<(), ServerFnError> {
    crate::server::auth::current_user_impl(headers)?
        .map(|_| ())
        .ok_or_else(|| ServerFnError::Response("You must be signed in to perform searches.".into()))
}


#[server(cookies: dioxus::fullstack::HeaderMap)]
pub async fn search_all(query: String) -> Result<Vec<SearchResult>, ServerFnError> {
    require_search_session(&cookies)?;
    let mut conn = crate::server::db::establish_connection();
    crate::server::search::gather_search_results(&query, &mut conn)
        .map_err(|error| ServerFnError::Response(format!("Could not perform search: {error}")))
}