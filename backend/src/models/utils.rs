use serde::{Deserialize, Serialize};
use crate::models::{NewProduct, NewVariant};


#[derive(Default, Serialize, Deserialize)]
pub struct ProductFilters {
    pub name: Option<String>,
    pub cost_ge: Option<f64>,
    pub cost_le: Option<f64>,
    pub is_active: Option<bool>
}

impl ProductFilters {
    pub fn is_empty(&self) -> bool {
        self.name.is_none() && 
        self.cost_ge.is_none() && 
        self.cost_le.is_none() && 
        self.is_active.is_none()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewVariantValue {
    pub variant: NewVariant,
    pub values: Vec<Option<String>>
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct NewCompleteProduct {
    pub product: NewProduct,
    pub variants: Vec<NewVariantValue>
}