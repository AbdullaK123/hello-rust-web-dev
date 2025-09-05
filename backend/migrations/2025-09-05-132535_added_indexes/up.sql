-- Your SQL goes here
CREATE INDEX idx_product_variants_product_id ON product_variants(product_id);
CREATE INDEX idx_product_variants_variant_id ON product_variants(variant_id);

-- First, drop the existing foreign key constraint if it exists.
-- The name 'product_variants_product_id_fkey' is a convention, replace if yours is different.
ALTER TABLE product_variants
DROP CONSTRAINT IF EXISTS product_variants_product_id_fkey;

-- Now, add the foreign key constraint with ON DELETE CASCADE.
-- Assumes 'products' table with 'id' primary key.
ALTER TABLE product_variants
ADD CONSTRAINT product_variants_product_id_fkey
FOREIGN KEY (product_id)
REFERENCES products(id)
ON DELETE CASCADE;

-- same for variant id
ALTER TABLE product_variants
DROP CONSTRAINT IF EXISTS product_variants_variant_id_fkey;

ALTER TABLE product_variants
ADD CONSTRAINT product_variants_variant_id_fkey
FOREIGN KEY (variant_id)
REFERENCES variants(id)
ON DELETE CASCADE;
