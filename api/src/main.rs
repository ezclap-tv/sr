use bytes::Bytes;
use warp::Filter;

fn get_address() -> ([u8; 4], u16) {
  if cfg!(debug_assertions) {
    ([127, 0, 0, 1], 8080)
  } else {
    let port = std::env::var("PORT")
      .expect("PORT must be set")
      .parse()
      .expect("PORT must be a number");
    ([0, 0, 0, 0], port)
  }
}

#[tokio::main]
async fn main() {
  env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

  let cors = warp::cors().allow_any_origin().build();

  let hello_world = warp::path::end().map(|| "Hello, world!").with(cors.clone());
  let echo = warp::post()
    .and(warp::path("echo"))
    .and(warp::body::bytes())
    .map(|data: Bytes| warp::reply::Response::new(warp::hyper::Body::from(data)))
    .with(cors);

  let routes = hello_world.or(echo);

  warp::serve(routes).run(get_address()).await;
}
