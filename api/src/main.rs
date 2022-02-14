use api::common::config::Config;
use structopt::StructOpt;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
  if std::env::var("RUST_LOG").is_err() {
    std::env::set_var("RUST_LOG", "info,actix_web=debug"); // actix_web=debug enables error logging
  }
  env_logger::init();

  dotenv::dotenv()?;

  let config = Config::from_args_safe()?;
  log::info!("Starting server on 0.0.0.0:{}", config.port);
  let socket = std::net::TcpListener::bind(("0.0.0.0", config.port))?;
  let server = api::start(socket, config).await?;
  Ok(server.await?)
}
