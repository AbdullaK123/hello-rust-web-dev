// @generated automatically by Diesel CLI.

diesel::table! {
    product_variants (id) {
        id -> Uuid,
        product_id -> Uuid,
        variant_id -> Uuid,
        value -> Nullable<Varchar>,
    }
}

diesel::table! {
    products (id) {
        id -> Uuid,
        name -> Varchar,
        cost -> Float8,
        active -> Bool,
    }
}

diesel::table! {
    variants (id) {
        id -> Uuid,
        name -> Varchar,
    }
}

diesel::joinable!(product_variants -> products (product_id));
diesel::joinable!(product_variants -> variants (variant_id));

diesel::allow_tables_to_appear_in_same_query!(
    product_variants,
    products,
    variants,
);
