use serde::{Serialize, Deserialize};

#[derive(Queryable, Serialize, Deserialize)]
pub struct Product {
    pub id: i32,
    pub price: f64,
    pub title: String,
    pub body: String,
    pub published: bool,
}
