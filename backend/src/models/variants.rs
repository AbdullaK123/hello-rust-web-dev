use diesel::{Insertable, Queryable, Identifiable};
use diesel::prelude::*;
use serde::{Serialize, Deserialize};
use uuid::Uuid;
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
    pub id: Uuid,
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
    pub variant_id: Uuid,
    pub product_id: Uuid,
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
    pub variant_id: Option<Uuid>,
    pub product_id: Option<Uuid>,
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
    pub id: Uuid,
    pub variant_id: Uuid,
    pub product_id: Uuid,
    pub value: Option<String>
}