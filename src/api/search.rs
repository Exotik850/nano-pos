use crate::Route;
#[cfg(feature = "server")]
use crate::api::auth::Session;
#[cfg(feature = "server")]
use crate::server::db::DbHandle;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct SearchResult {
    pub route: Route,
    pub title: String,
    pub description: String,
}

#[server(db: DbHandle, _session: Session)]
pub async fn search_all(query: String) -> Result<Vec<SearchResult>, ServerFnError> {
    let mut conn = db.get().map_err(ServerFnError::Response)?;
    crate::server::search::gather_search_results(&query, &mut conn)
        .map_err(|error| ServerFnError::Response(format!("Could not perform search: {error}")))
}
