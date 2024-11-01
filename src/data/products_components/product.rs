use rocket::serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    pub name: String,
    pub description: Option<String>,
    pub primary_image_id: Option<i32>,
    pub price: i32,
    pub stock_quantity: Option<i32>,
    pub size_id: Option<i32>,
    pub category_id: Option<i32>,
}