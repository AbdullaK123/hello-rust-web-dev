-- This file should undo anything in `up.sql`

-- Drop the foreign key constraints with ON DELETE CASCADE
ALTER TABLE product_variants
DROP CONSTRAINT IF EXISTS product_variants_variant_id_fkey;

ALTER TABLE product_variants
DROP CONSTRAINT IF EXISTS product_variants_product_id_fkey;

-- Recreate the original foreign key constraints without ON DELETE CASCADE
-- (assuming the original constraints existed without CASCADE)
ALTER TABLE product_variants
ADD CONSTRAINT product_variants_product_id_fkey
FOREIGN KEY (product_id)
REFERENCES products(id);

ALTER TABLE product_variants
ADD CONSTRAINT product_variants_variant_id_fkey
FOREIGN KEY (variant_id)
REFERENCES variants(id);

-- Drop the indexes
DROP INDEX IF EXISTS idx_product_variants_variant_id;
DROP INDEX IF EXISTS idx_product_variants_product_id;
