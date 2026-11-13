#[cfg(feature = "server")]
use crate::api::auth::Session;
#[cfg(feature = "server")]
use crate::server::db::DbHandle;
use crate::Route;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
pub struct SearchResults {
    pub named: HashMap<String, Vec<SearchResult>>,
    pub unnamed: Vec<SearchResult>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
pub struct SearchResult {
    pub route: Route,
    pub title: String,
    pub description: String,
    pub category: Option<String>,
}

#[cfg(feature = "server")]
impl SearchResult {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_route(mut self, route: Route) -> Self {
        self.route = route;
        self
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = description.into();
        self
    }

    pub fn with_category(mut self, category: impl Into<String>) -> Self {
        self.category = Some(category.into());
        self
    }
}

#[server(db: DbHandle, _session: Session)]
pub async fn search_all(query: String) -> Result<SearchResults, ServerFnError> {
    let mut conn = db.get().map_err(ServerFnError::Response)?;
    crate::server::search::gather_search_results(&query, &mut conn)
        .map_err(|error| ServerFnError::Response(format!("Could not perform search: {error}")))
}
