use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn welcome() -> impl Responder {
    HttpResponse::Ok().body("Hello world, welcome to aayojak!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(welcome).service(echo))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}