
use std::time::SystemTime;

use chrono::Utc;
use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Burger {
    pub _id : ObjectId,
    pub str_burger: String,
    pub str_tags: Option<String>,
    pub str_category: Option<String>,
    pub is_vegan: bool,
    pub str_instructions: Option<String>,
    pub str_burgerimg: String,
    pub str_ingredients: Vec<String>,
    pub str_measuremetric: String,
    pub str_measureimperial: String,
    pub date_modified: DateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BurgerReq {
    pub str_burger: String,
    pub str_tags: Option<String>,
    pub str_category: Option<String>,
    pub is_vegan: bool,
    pub str_instructions: Option<String>,
    pub str_burgerimg: String,
    pub str_ingredients: Vec<String>,
    pub str_measuremetric: String,
    pub str_measureimperial: String,
}

//Implementing conversion from request input
impl TryFrom<BurgerReq> for Burger {
    type Error = Box<dyn std::error::Error>;

    fn try_from(item: BurgerReq) -> Result<Self, Self::Error> {
        let now: SystemTime = Utc::now().into();
       Ok(Self {
        _id: ObjectId::new(),
        str_burger: item.str_burger,
        str_tags: item.str_tags,
        str_category: item.str_category,
        is_vegan: item.is_vegan,
        str_instructions: item.str_instructions,
        str_burgerimg: item.str_burgerimg,
        str_ingredients: item.str_ingredients,
        str_measuremetric: item.str_measuremetric,
        str_measureimperial: item.str_measureimperial,
        date_modified: DateTime::from_system_time(now),
       })
    }
}