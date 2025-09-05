-- Remove auto-increment defaults
ALTER TABLE products ALTER COLUMN id DROP DEFAULT;
ALTER TABLE variants ALTER COLUMN id DROP DEFAULT;
ALTER TABLE product_variants ALTER COLUMN id DROP DEFAULT;

-- Drop sequences
DROP SEQUENCE IF EXISTS products_id_seq;
DROP SEQUENCE IF EXISTS variants_id_seq;
DROP SEQUENCE IF EXISTS product_variants_id_seq;
