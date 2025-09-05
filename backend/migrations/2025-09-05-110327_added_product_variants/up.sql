-- Your SQL goes here
CREATE TABLE variants (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR NOT NULL
);

CREATE TABLE product_variants (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    product_id UUID NOT NULL,
    variant_id UUID NOT NULL,
    value VARCHAR,
    FOREIGN KEY (variant_id) REFERENCES variants(id),
    FOREIGN KEY (product_id) REFERENCES products(id)
)