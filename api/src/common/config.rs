use secrecy::Secret;
use structopt::StructOpt;

fn parse_duration(src: &str) -> Result<chrono::Duration, humantime::DurationError> {
  let duration = src.parse::<humantime::Duration>()?;
  chrono::Duration::from_std(*duration).map_err(|_| humantime::DurationError::NumberOverflow)
}

#[derive(Debug, Clone, StructOpt)]
#[structopt(name = "api", about = "Song Request API", rename_all = "kebab-case")]
pub struct Config {
  #[structopt(
    long,
    env = "SR_API_GOOGLE_API_KEY",
    help = "Google API key, follow the steps at https://developers.google.com/youtube/v3/getting-started#before-you-start to obtain one"
  )]
  pub youtube_key: Secret<String>,
  #[structopt(long, env = "SR_API_DATABASE_URL", help = "PostgreSQL database URL")]
  pub database_url: String,
  #[structopt(long, env = "SR_API_PORT", help = "Port to bind on")]
  pub port: u16,
  #[structopt(
    long,
    env = "SR_API_PLAYLIST_REFRESH_INTERVAL",
    help = "How often playlists should be re-fetched",
    parse(try_from_str = parse_duration)
  )]
  pub playlist_refresh_interval: chrono::Duration,
}
