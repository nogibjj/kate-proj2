use randomfood::make_reqwest;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("What to eat for dinner?")
}

#[get("/meal")]
async fn food() -> impl Responder {
    let meal = make_reqwest().await;
    // HttpResponse::Ok().body(meal["mealname"].to_string())
    HttpResponse::Ok().json(meal)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //add a print message to the console that the service is running
    println!("Running the service");
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(food)

    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
