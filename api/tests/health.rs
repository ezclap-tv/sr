#[actix_rt::test]
async fn health_check() -> anyhow::Result<()> {
  let base_url = spawn()?;

  let response = reqwest::get(&format!("{base_url}/health")).await?;

  assert!(response.status().is_success());
  assert_eq!(Some(0), response.content_length());

  Ok(())
}

pub fn spawn() -> anyhow::Result<String> {
  let socket = std::net::TcpListener::bind("127.0.0.1:0")?;
  let port = socket.local_addr()?.port();
  tokio::spawn(api::run(socket)?);
  Ok(format!("http://127.0.0.1:{port}"))
}
