use std::sync::{Arc, LazyLock, Mutex};

use diesel::{Connection, PgConnection};
use diesel_migrations::{EmbeddedMigrations, MigrationHarness, embed_migrations};
use dioxus::logger::tracing::warn;

// thread_local! {
//   pub static DB: PgConnection = {
//     if let Err(e) = dotenvy::dotenv() {
//       warn!("Failed to load .env file: {}", e);
//     };
//     let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
//     let conn = PgConnection::establish(&db_url).expect("Error connecting to database");
//     conn
//   };
// }
const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub static DB: LazyLock<Arc<Mutex<PgConnection>>> = LazyLock::new(|| {
    if let Err(e) = dotenvy::dotenv() {
        warn!("Failed to load .env file: {}", e);
    };
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut conn = PgConnection::establish(&db_url).expect("Error connecting to database");
    conn.run_pending_migrations(MIGRATIONS).expect("Could not run migrations");
    Arc::new(Mutex::new(conn))
});

pub(crate) fn establish_connection<'a>() -> std::sync::MutexGuard<'a, PgConnection> {
    DB.lock()
        .expect("Failed to acquire database connection lock")
}

