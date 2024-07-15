use actix_web::{post, get, web::{Data, Json, Path}, HttpResponse};

use crate::models::ingredient_model::{Ingredient, IngredientReq};
use crate::services::db::Database;

//Create an ingredient
#[post("/api/create_ingredient")]
pub async fn create_ingr(db: Data<Database>, req: Json<IngredientReq>) -> HttpResponse {
    match db
    .create_ingredient(
        Ingredient::try_from(IngredientReq{
            str_name: req.str_name.clone(),
            str_desc: req.str_desc.clone(),
            str_type: req.str_type.clone(),
            })
            .expect("Error converting IngredientRequest to Ingredient.")
    )
    .await {
        Ok(ingr) => HttpResponse::Ok().json(ingr),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

//Listing all info about ingredient with given id
#[get("/api/ingredient/lookup/{id}")]
pub async fn ingredient_id_info(db:Data<Database>, p: Path<(String,)>)-> HttpResponse{
    let id = p.into_inner().0;
    match db.ingredient_id_info(id.as_str()).await {
        Ok(ingredient) => HttpResponse::Ok().json(ingredient),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

//Search ingredients with given name
#[get("/api/ingredient/search/{name}")]
pub async fn search_ingredients(db:Data<Database>, p: Path<(String,)>)-> HttpResponse{
    let name = p.into_inner().0;
    match db.search_ingredients(name.as_str()).await {
        Ok(ingredients) => HttpResponse::Ok().json(ingredients),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}

//List all ingredients in database
#[get("/api/ingredient/list")]
pub async fn list_ingredients(db:Data<Database>)-> HttpResponse{
    match db.list_ingredients().await {
        Ok(ingredients) => HttpResponse::Ok().json(ingredients),
        Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
    }
}