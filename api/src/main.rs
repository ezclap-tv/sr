use anyhow::Result;

#[actix_web::main]
async fn main() -> Result<()> {
  if std::env::var("RUST_LOG").is_err() {
    std::env::set_var("RUST_LOG", "info,actix_web=debug"); // actix_web=debug enables error logging
  }
  env_logger::init();

  api::run("127.0.0.1:8000".parse().unwrap()).await
}
