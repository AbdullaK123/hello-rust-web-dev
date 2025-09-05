use diesel::{Insertable, Queryable, Identifiable};
use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use crate::schema::*;

#[derive(
    Insertable, 
    Debug, 
    Clone,
    Serialize,
    Deserialize
)]
#[diesel(table_name = variants)]
pub struct NewVariant {
    pub name: String
}

#[derive(
    AsChangeset, 
    Debug, 
    Clone,
    Serialize,
    Deserialize
)]
#[diesel(table_name = variants)]
pub struct VariantUpdate {
    pub name: Option<String>
}

#[derive(
    Queryable, 
    Selectable, 
    Identifiable, 
    Debug, 
    Serialize, 
    Deserialize,
    Clone
)]
#[diesel(table_name = variants)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Variant {
    pub id: i32,
    pub name: String
}

#[derive(
    Insertable, 
    Debug, 
    Clone,
    Serialize,
    Deserialize
)]
#[diesel(table_name = product_variants)]
pub struct NewProductVariant {
    pub variant_id: i32,
    pub product_id: i32,
    pub value: String
}

#[derive(
    AsChangeset, 
    Debug, 
    Clone,
    Serialize,
    Deserialize
)]
#[diesel(table_name = product_variants)]
pub struct ProductVariantUpdates {
    pub variant_id: Option<i32>,
    pub product_id: Option<i32>,
    pub value: Option<String>
}

#[derive(
    Queryable, 
    Selectable, 
    Identifiable, 
    Debug, 
    Serialize, 
    Deserialize,
    Clone
)]
#[diesel(table_name = product_variants)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct ProductVariant {
    pub id: i32,
    pub variant_id: i32,
    pub product_id: i32,
    pub value: Option<String>
}