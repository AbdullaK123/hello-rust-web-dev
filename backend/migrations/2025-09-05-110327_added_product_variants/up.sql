-- Your SQL goes here
CREATE TABLE variants (
    id INT PRIMARY KEY,
    name VARCHAR NOT NULL
);

CREATE TABLE product_variants (
    id INT PRIMARY KEY,
    product_id INT NOT NULL,
    variant_id INT NOT NULL,
    value VARCHAR,
    FOREIGN KEY (variant_id) REFERENCES variants(id),
    FOREIGN KEY (product_id) REFERENCES products(id)
)