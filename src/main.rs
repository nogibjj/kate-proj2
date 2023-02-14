use randomfood::make_reqwest;
use actix_web::{get, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello World Random Food!")
}
#[get("/food")]
async fn food() -> impl Responder {
    let meal = make_reqwest().await;
    // print_dictionary(dictionary);
 //http response with mealname
    HttpResponse::Ok().body(meal["mealname"].to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //add a print message to the console that the service is running
    println!("Running the service");
    HttpServer::new(|| {
        App::new()
            .service(food)

    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
