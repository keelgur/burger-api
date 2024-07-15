use std::{env, str::FromStr, time::SystemTime};

use chrono::Utc;
use futures_util::StreamExt;
use mongodb::{bson::{doc, from_document, oid::ObjectId, DateTime, Document,Bson}, error::Error, results::InsertOneResult, Client, Collection};

use crate::models::{burger_model::Burger, ingredient_model::Ingredient};

pub struct Database{
    burger: Collection<Burger>,
    ingredient: Collection<Ingredient>
}

impl Database {
    pub async fn init() -> Self {
        let uri = match env::var("MONGO_URI"){
            Ok(v) => v.to_string(),
            Err(_) => "mongodb://localhost:27017/?directConnection=true".to_string(),
        };

        let client = Client::with_uri_str(uri).await.unwrap();
        let db = client.database("burger_tray");

        let burger: Collection<Burger> = db.collection("burger");
        let ingredient: Collection<Ingredient> = db.collection("ingredient");

        Database {
            burger,
            ingredient,
        }

    }

    pub async fn create_burger(&self, bur: Burger) -> Result<InsertOneResult, Error> {
        let res = self
        .burger
        .insert_one(bur)
        .await
        .ok()
        .expect("Error creating a burger");
      
      Ok(res)
    }

    pub async fn create_ingredient(&self, ing: Ingredient) -> Result<InsertOneResult, Error> {
        let res = self
        .ingredient
        .insert_one(ing)
        .await
        .ok()
        .expect("Error creating an ingredient");
      
      Ok(res)
    }

    pub async fn search_burgers(&self,name: &str) -> Result<Vec<Burger>,Error> {
        let mut res = self
        .burger
        .aggregate(vec![
            doc! {
                "$match": {
                "str_burger" : doc!{
                    "$regex" : name
                }
              }
            },
        ])
        .await
        .ok()
        .expect("Error searching for burgers");
     let mut burgers: Vec<Burger> = Vec::new();

     while let Some(result) = res.next().await {
        match result {
            Ok(d) => {
                let br: Burger = from_document(d).expect("Error converting doc to a burger model");
                burgers.push(br);
            },
            Err(err) => panic!("Error getting burger: {}", err)
        }
     }
     Ok(burgers)
    }

    pub async fn search_byletter(&self,l: &str) -> Result<Vec<Burger>,Error> {
        let mut res = self
        .burger
        .aggregate(vec![
            doc! {
                "$match": {
                "str_burger" : doc!{
                    "$regex" : l
                }
              }
            },
        ])
        .await
        .ok()
        .expect("Error searching for burgers");
     let mut burgers: Vec<Burger> = Vec::new();

     while let Some(result) = res.next().await {
        match result {
            Ok(d) => {
                let br: Burger = from_document(d).expect("Error converting doc to a burger model");
                burgers.push(br);
            },
            Err(err) => panic!("Error getting burger: {}", err)
        }
     }
     Ok(burgers)
    }

    pub async fn burger_id_info(&self,id: &str) -> Result<Vec<Burger>,Error> {

        let mut res = self
        .burger
        .find(doc! {"_id": ObjectId::from_str(id).expect("Failed to parse burger id")})
        .await
        .ok()
        .expect("Error finding burger");
        let mut burger: Vec<Burger> = Vec::new();

        while let Some(result) = res.next().await {
            match result {
                Ok(d) => burger.push(d),
                Err(err) => panic!("Error finding burger by {} id: {}", &id, err),
            }
        }

        Ok(burger)
    }

    pub async fn search_byingr(&self,ingr: &str) -> Result<Vec<Document>,Error> {
        let mut res = self
        .burger
        .aggregate(vec![
            doc! {
                "$match": {
                "str_ingredients" : {
                    "$elemMatch" : {"$regex":ingr}
                }
              }
            },
            doc!{
                "$project": {
                    "str_burger": 1,
                    "str_burgerimg": 1,
                    "_id": 1,
                  }
            }
        ])
        .await
        .ok()
        .expect("Error searching for burgers");
     let mut burgers: Vec<Document> = Vec::new();

     while let Some(result) = res.next().await {
        match result {
            Ok(d) => {
                burgers.push(d);
            },
            Err(err) => panic!("Error getting burger: {}", err)
        }
     }
     Ok(burgers)
    }

    pub async fn filter_byvegan(&self, b: bool) -> Result<Vec<Document>,Error> {
        let mut res = self
        .burger
        .aggregate(vec![
            doc! {
                "$match": {
                "is_vegan" : {
                    "$not" : {"$eq" : b}
                }
              }
            },
            doc!{
                "$project": {
                    "str_burger": 1,
                    "str_burgerimg": 1,
                    "_id": 1,
                  }
            }
        ])
        .await
        .ok()
        .expect("Error searching for burgers");
     let mut burgers: Vec<Document> = Vec::new();

     while let Some(result) = res.next().await {
        match result {
            Ok(d) => {
                burgers.push(d);
            },
            Err(err) => panic!("Error getting burger: {}", err)
        }
     }
     Ok(burgers)
    }

    pub async fn filter_bycat(&self, category: &str) -> Result<Vec<Document>,Error> {
        let mut res = self
        .burger
        .aggregate(vec![
            doc! {
                "$match": {
                    "str_category" : {
                        "$regex":category
                    }
              }
            },
            doc!{
                "$project": {
                    "str_burger": 1,
                    "str_burgerimg": 1,
                    "_id": 1,
                  }
            }
        ])
        .await
        .ok()
        .expect("Error searching for burgers");
     let mut burgers: Vec<Document> = Vec::new();

     while let Some(result) = res.next().await {
        match result {
            Ok(d) => {
                burgers.push(d);
            },
            Err(err) => panic!("Error getting burger: {}", err)
        }
     }
     Ok(burgers)
    }

    pub async fn get_random_burger(&self) -> Result<Vec<Burger>,Error> {
        let mut res = self
        .burger
        .aggregate(vec![
            doc! {
                "$sample": {
                    "size": 1
              }
            }
        ])
        .await
        .ok()
        .expect("Error searching for burgers");
     let mut burgers: Vec<Burger> = Vec::new();

     while let Some(result) = res.next().await {
        match result {
            Ok(d) => {
                let br: Burger = from_document(d).expect("Error converting doc to a burger model");
                burgers.push(br);
            },
            Err(err) => panic!("Error getting burger: {}", err)
        }
     }
     Ok(burgers)
    }

    pub async fn get_random_10_burgers(&self) -> Result<Vec<Burger>,Error> {
        let mut res = self
        .burger
        .aggregate(vec![
            doc! {
                "$sample": {
                    "size": 10
              }
            }
        ])
        .await
        .ok()
        .expect("Error searching for burgers");
     let mut burgers: Vec<Burger> = Vec::new();

     while let Some(result) = res.next().await {
        match result {
            Ok(d) => {
                let br: Burger = from_document(d).expect("Error converting doc to a burger model");
                burgers.push(br);
            },
            Err(err) => panic!("Error getting burger: {}", err)
        }
     }
     Ok(burgers)
    }

    pub async fn get_latest(&self) -> Result<Vec<Document>,Error> {
        let now: SystemTime = Utc::now().into();
        let mut res = self
        .burger
        .aggregate(vec![
            doc! {
                "$match": {
                    "date_modified": {
                        "$lte": DateTime::from_system_time(now),
                    }
                }
            },
            doc!{
                "$group":{
                    "_id":Bson::Null,
                "latest_burgers":{"$firstN": {"input":doc!{"name":"$str_burger","image":"$str_burgerimg","id":"$_id"},"n":3 }}
                }
             },
            ])
        .await
        .ok()
        .expect("Error searching for burgers");
     let mut burgers: Vec<Document> = Vec::new();

     while let Some(result) = res.next().await {
        match result {
            Ok(d) => {
                burgers.push(d);
            },
            Err(err) => panic!("Error getting burger: {}", err)
        }
     }
     Ok(burgers)
    }

    pub async fn ingredient_id_info(&self,id: &str) -> Result<Vec<Ingredient>,Error> {
        let mut res = self
        .ingredient
        .find(doc! {"_id": ObjectId::from_str(id).expect("Failed to parse burger id")})
        .await
        .ok()
        .expect("Error finding ingredient");
        let mut ingredient: Vec<Ingredient> = Vec::new();

        while let Some(result) = res.next().await {
            match result {
                Ok(d) => ingredient.push(d),
                Err(err) => panic!("Error finding ingredient by {} id: {}", &id, err),
            }
        }

        Ok(ingredient)
    }

    pub async fn search_ingredients(&self,name: &str) -> Result<Vec<Ingredient>,Error> {
        let mut res = self
        .ingredient
        .aggregate(vec![
            doc! {
                "$match": {
                "str_name" : doc!{
                    "$regex" : name
                }
              }
            },
        ])
        .await
        .ok()
        .expect("Error searching for ingredients");
     let mut ingredients: Vec<Ingredient> = Vec::new();

     while let Some(result) = res.next().await {
        match result {
            Ok(d) => {
                let br: Ingredient = from_document(d).expect("Error converting doc to an ingredient model");
                ingredients.push(br);
            },
            Err(err) => panic!("Error getting ingredient: {}", err)
        }
     }
     Ok(ingredients)
    }

    pub async fn list_ingredients(&self) -> Result<Vec<Document>,Error> {
        let mut res = self
        .ingredient
        .aggregate(vec![
            doc! {
                "$match": {
                "str_name": doc!{
                    "$exists": true
                }
              }
            },
            doc! {
                "$project": {
                "_id": 0,
                "str_name": 1,
              }
            },
        ])
        .await
        .ok()
        .expect("Error searching for ingredients");
     let mut ingredients: Vec<Document> = Vec::new();

     while let Some(result) = res.next().await {
        match result {
            Ok(d) => {
                ingredients.push(d);
            },
            Err(err) => panic!("Error getting ingredient: {}", err)
        }
     }
     Ok(ingredients)
    }
    
}