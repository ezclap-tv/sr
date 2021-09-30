use actix_cors::Cors;
use actix_web::{get, middleware::Logger, post, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(body: String) -> impl Responder {
    HttpResponse::Ok().body(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let address = if cfg!(debug_assertions) {
        ("127.0.0.1", 8080)
    } else {
        let port = std::env::var("PORT")
            .expect("PORT must be set")
            .parse()
            .expect("PORT must be a number");
        ("0.0.0.0", port)
    };

    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive())
            .wrap(Logger::default())
            .service(hello)
            .service(echo)
    })
    .bind(address)
    .expect(&format!("Failed to bind on {}:{}", address.0, address.1))
    .run()
    .await
}
