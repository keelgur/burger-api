use actix_web::{get, App, HttpResponse, HttpServer, Responder, web::Data};
use routes::{burger_route::{burger_id_info, create_burger, filter_bycat, filter_byvegan, get_latest, get_random_10_burgers, get_random_burger, search_burgers, search_byingr, search_byletter}, ingredient_route::{create_ingr, ingredient_id_info, list_ingredients, search_ingredients}};
use services::db::Database;

mod models;
mod services;
mod routes;

#[get("/")]
async fn rootpage() -> impl Responder {
    HttpResponse::Ok().body("Welcome to\nBurger API!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = Database::init().await;
    let db_dat = Data::new(db);

    HttpServer::new(move || App::new().app_data(db_dat.clone())
    .service(rootpage)
    .service(create_burger)
    .service(create_ingr)
    .service(search_burgers)
    .service(search_byingr)
    .service(filter_bycat)
    .service(filter_byvegan)
    .service(search_byletter)
    .service(search_ingredients)
    .service(burger_id_info)
    .service(get_random_burger)
    .service(get_random_10_burgers)
    .service(get_latest)
    .service(ingredient_id_info)
    .service(list_ingredients)
)
    .bind(("localhost", 8000))?
    .run()
    .await
}
