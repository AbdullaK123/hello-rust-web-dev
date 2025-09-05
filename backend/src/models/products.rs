use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::*;

#[derive(
    Insertable, 
    Debug, 
    Clone, 
    Default,
    Serialize,
    Deserialize
)]
#[diesel(table_name = products)]
pub struct NewProduct {
    pub name: String,
    pub cost: f64,
    pub active: bool,
}

#[derive(AsChangeset, Debug, Clone, Serialize, Deserialize)]
#[diesel(table_name = products)]
pub struct ProductUpdates {
    pub name: Option<String>,
    pub cost: Option<f64>,
    pub active: Option<bool>
}

#[derive(
    Identifiable, 
    Queryable,
    Selectable, 
    Debug, 
    Clone, 
    Deserialize, 
    Serialize
)]
#[diesel(table_name = products)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub cost: f64,
    pub active: bool,
}