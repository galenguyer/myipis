use actix_web::http::{StatusCode};
use actix_web::{
    middleware, web, App, HttpRequest, HttpResponse, HttpServer, Result,
};
use std::{env};

async fn index(req: HttpRequest) -> Result<HttpResponse> {
    let connection_info = req.connection_info().clone();

    let ip_port = match connection_info.realip_remote_addr() {
        Some(ip) => ip.to_owned(),
        None => String::from(""),
    };

    let ip = ip_port
        .split(":")
        .map(|x| x.to_owned())
        .collect::<Vec<String>>()
        .get(0)
        .unwrap()
        .to_owned();
    // response
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/plain; charset=utf-8")
        .body(ip))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(index))
            .route("/ip", web::get().to(index))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
