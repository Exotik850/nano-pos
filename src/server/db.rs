use std::sync::{Arc, Mutex, MutexGuard};

use diesel::{Connection, PgConnection};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use dioxus::{
    Result,
    fullstack::{
        FullstackContext,
        extract::{FromRef, FromRequestParts},
    },
    logger::tracing::warn,
};

const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

#[derive(Clone)]
pub struct DbHandle(pub Arc<Mutex<PgConnection>>);

impl std::ops::Deref for DbHandle {
    type Target = Arc<Mutex<PgConnection>>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DbHandle {
    pub fn new() -> Result<Self> {
        if let Err(e) = dotenvy::dotenv() {
            warn!("Failed to load .env file: {}", e);
        };
        let db_url = std::env::var("DATABASE_URL")?;
        let mut conn = PgConnection::establish(&db_url)?;
        conn.run_pending_migrations(MIGRATIONS)
            .expect("Could not run migrations");
        Ok(Self(Arc::new(Mutex::new(conn))))
    }

    pub fn get(&self) -> Result<MutexGuard<'_, PgConnection>, String> {
        self.0
            .lock()
            .map_err(|error| format!("Database lock poisoned: {error}"))
    }
}

impl FromRef<FullstackContext> for DbHandle {
    fn from_ref(input: &FullstackContext) -> Self {
        input
            .extension::<DbHandle>()
            .expect("DB connection initialized in server")
    }
}

impl<S> FromRequestParts<S> for DbHandle
where
    S: Send + Sync,
    DbHandle: FromRef<S>,
{
    type Rejection = !;
    async fn from_request_parts(
        _: &mut dioxus::prelude::dioxus_server::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        Ok(Self::from_ref(state))
    }
}
