use actix_web::{get, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
#[get("/{id}")]
async fn id() -> impl Responder {
    HttpResponse::Ok().body("Hello from id!")
}

#[get("/create")]
async fn create() -> impl Responder {
    HttpResponse::Ok().body("Hello from create!")
}

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    println!("{:?}", req);
    format!("Hello {}!", &name)
    // HttpResponse::Ok().body("Hey there!")
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    HttpServer::new(|| {
        App::new()
            .route("/hello", web::get().to(greet))
            .service(create)
            .service(id)
        // .route("/{name}", web::get().to(greet))
        // .route("/create", web::get().to(greet))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
