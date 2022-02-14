pub mod playlists;
pub mod songs;

pub type Database = sqlx::PgPool;

/// Connect to the database via a URI.
#[inline]
pub async fn connect(uri: &str) -> sqlx::Result<Database> {
  Database::connect(uri).await
}

/// Connect to the database with individual parameters.
#[inline]
pub async fn connect_with(host: &str, port: &str, name: &str, user: &str, password: &str) -> sqlx::Result<Database> {
  connect(&format!(
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
#[inline]
pub async fn connect_from_env() -> sqlx::Result<Database> {
  let host = std::env::var("DB_HOST").expect("Missing environment variable `DB_HOST`");
  let port = std::env::var("DB_PORT").expect("Missing environment variable `DB_PORT`");
  let name = std::env::var("DB_NAME").expect("Missing environment variable `DB_NAME`");
  let user = std::env::var("DB_USER").expect("Missing environment variable `DB_USER`");
  let password = std::env::var("DB_PASSWORD").expect("Missing environment variable `DB_PASSWORD`");
  connect_with(&host, &port, &name, &user, &password).await
}

#[cfg(test)]
mod tests {
  // TODO: connect to db without name, start a new logical db per test
  #[macro_export]
  macro_rules! db_test {
    ($name:ident, $tx:ident $body:block) => {
      #[actix_rt::test]
      #[cfg_attr(not(feature = "test-database"), ignore)]
      async fn $name() -> anyhow::Result<()> {
        let conn = db::connect_from_env().await?;
        let mut $tx = conn.begin().await?;

        $body

        $tx.rollback().await?;
        Ok(())
      }
    }
  }
}
