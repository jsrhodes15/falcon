use actix_web::{web, App, HttpRequest, HttpServer, HttpResponse, Responder};
use actix_web::dev::Server;
use std::net::TcpListener;

async fn health_check(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
}

pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new( || {
        App::new()
            .route("/health", web::get().to(health_check))

    })
        .listen(listener)?
        .run();

    Ok(server)
}