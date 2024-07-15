
use actix_web::{get, post, web::{Data, Json, Path}, HttpResponse};

use crate::models::burger_model::{Burger, BurgerReq};
use crate::services::db::Database;

//Create a document
#[post("/api/create_burger")]
pub async fn create_burger(db: Data<Database>, req: Json<BurgerReq>) -> HttpResponse {
    match db
    .create_burger(
        Burger::try_from(BurgerReq{
            str_burger: req.str_burger.clone(),
            str_tags: req.str_tags.clone(),
            str_category: req.str_category.clone(),
            is_vegan: req.is_vegan.clone(),
            str_instructions: req.str_instructions.clone(),
            str_burgerimg: req.str_burgerimg.clone(),
            str_ingredients: req.str_ingredients.clone(),
            str_measuremetric: req.str_measuremetric.clone(),
            str_measureimperial: req.str_measureimperial.clone()
            })
            .expect("Error converting BurgerRequest to Burger.")
    )
    .await {
        Ok(bur) => HttpResponse::Ok().json(bur),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

//Search documents with given name
#[get("/api/burger/search/{name}")]
pub async fn search_burgers(db:Data<Database>, p: Path<(String,)>)-> HttpResponse{
    let name = p.into_inner().0;
    match db.search_burgers(name.as_str()).await {
        Ok(burgers) => HttpResponse::Ok().json(burgers),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

//Search documents which names start with given letter
#[get("/api/burger/searchletter/{letter}")]
pub async fn search_byletter(db:Data<Database>, p: Path<(String,)>)-> HttpResponse{
    let letter = "^".to_owned()+p.into_inner().0.as_str();
    match db.search_byletter(letter.as_str()).await {
        Ok(burgers) => HttpResponse::Ok().json(burgers),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

//Listing all info about document with given id
#[get("/api/burger/lookup/{id}")]
pub async fn burger_id_info(db:Data<Database>, p: Path<(String,)>)-> HttpResponse{
    let id = p.into_inner().0;
    match db.burger_id_info(id.as_str()).await {
        Ok(burger) => HttpResponse::Ok().json(burger),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

//Get info of random document
#[get("/api/burger/random")]
pub async fn get_random_burger(db:Data<Database>)-> HttpResponse{
    match db.get_random_burger().await {
        Ok(burger) => HttpResponse::Ok().json(burger),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

//Get info of 10 random documents
#[get("/api/burger/randomselection")]
pub async fn get_random_10_burgers(db:Data<Database>)-> HttpResponse{
    match db.get_random_10_burgers().await {
        Ok(burgers) => HttpResponse::Ok().json(burgers),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

//Get info of 3 most recently added documents
#[get("/api/burger/latest")]
pub async fn get_latest(db:Data<Database>)-> HttpResponse{
    match db.get_latest().await {
        Ok(burgers) => HttpResponse::Ok().json(burgers),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

//Search documents with given ingredient
#[get("/api/burger/filteri/{ingr}")]
pub async fn search_byingr(db:Data<Database>, p: Path<(String,)>)-> HttpResponse{
    let ingr = p.into_inner().0;
    match db.search_byingr(ingr.as_str()).await {
        Ok(burgers) => HttpResponse::Ok().json(burgers),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

//Filter only is_vegan:true or is_vegan:false documents, based on given category
#[get("/api/burger/filterv/{diet}")]
pub async fn filter_byvegan(db:Data<Database>, p: Path<(String,)>)-> HttpResponse{
    let diet = p.into_inner().0;
    let b:bool;
    if diet=="Vegan"{
        b=false;
    }
    else if diet=="Non_Vegan"{
        b=true;
    }
    else{
        return HttpResponse::InternalServerError().body("error:Invalid filter to request");
    }
    match db.filter_byvegan(b).await {
        Ok(burgers) => HttpResponse::Ok().json(burgers),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

//Search documents with given category
#[get("/api/burger/filterc/{category}")]
pub async fn filter_bycat(db:Data<Database>, p: Path<(String,)>)-> HttpResponse{
    let category = p.into_inner().0;
    match db.filter_bycat(category.as_str()).await {
        Ok(burgers) => HttpResponse::Ok().json(burgers),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}