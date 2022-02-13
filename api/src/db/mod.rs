pub mod playlists;
pub mod songs;

use sqlx::PgPool;

// TODO: fix queries (schema changed)
// TODO: write tests for all queries (?) - even the simple ones

/// Connect to the database.
pub async fn connect(host: &str, port: &str, name: &str, user: &str, password: &str) -> sqlx::Result<PgPool> {
  PgPool::connect(&format!(
    "postgres://{host}:{port}/{name}?user={user}&password={password}"
  ))
  .await
}

/// Connect to the database using environment variables.
///
/// Expects the following environment variables:
/// - `DB_HOST`
/// - `DB_PORT`
/// - `DB_NAME`
/// - `DB_USER`
/// - `DB_PASSWORD`
///
/// Panics if any of them are unavailable.
pub async fn connect_from_env() -> sqlx::Result<PgPool> {
  let host = std::env::var("DB_HOST").expect("Missing environment variable `DB_HOST`");
  let port = std::env::var("DB_PORT").expect("Missing environment variable `DB_PORT`");
  let name = std::env::var("DB_NAME").expect("Missing environment variable `DB_NAME`");
  let user = std::env::var("DB_USER").expect("Missing environment variable `DB_USER`");
  let password = std::env::var("DB_PASSWORD").expect("Missing environment variable `DB_PASSWORD`");
  connect(&host, &port, &name, &user, &password).await
}
