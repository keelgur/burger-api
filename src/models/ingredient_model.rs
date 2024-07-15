use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Ingredient {
    pub _id : ObjectId,
    pub str_name: String,
    pub str_desc: String,
    pub str_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IngredientReq {
    pub str_name: String,
    pub str_desc: String,
    pub str_type: String,
}

impl TryFrom<IngredientReq> for Ingredient {
    type Error = Box<dyn std::error::Error>;

    fn try_from(item: IngredientReq) -> Result<Self, Self::Error> {
        
       Ok(Self {
        _id: ObjectId::new(),
        str_name: item.str_name,
        str_desc: item.str_desc,
        str_type: item.str_type,
       })
    }
}