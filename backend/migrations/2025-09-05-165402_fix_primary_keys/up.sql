-- Create sequences if they don't exist
DO $$
BEGIN
    IF NOT EXISTS (SELECT 1 FROM pg_sequences WHERE sequencename = 'products_id_seq') THEN
        CREATE SEQUENCE products_id_seq OWNED BY products.id;
        PERFORM setval('products_id_seq', COALESCE(MAX(id), 0) + 1, false) FROM products;
        ALTER TABLE products ALTER COLUMN id SET DEFAULT nextval('products_id_seq');
    END IF;
    
    IF NOT EXISTS (SELECT 1 FROM pg_sequences WHERE sequencename = 'variants_id_seq') THEN
        CREATE SEQUENCE variants_id_seq OWNED BY variants.id;
        PERFORM setval('variants_id_seq', COALESCE(MAX(id), 0) + 1, false) FROM variants;
        ALTER TABLE variants ALTER COLUMN id SET DEFAULT nextval('variants_id_seq');
    END IF;
    
    IF NOT EXISTS (SELECT 1 FROM pg_sequences WHERE sequencename = 'product_variants_id_seq') THEN
        CREATE SEQUENCE product_variants_id_seq OWNED BY product_variants.id;
        PERFORM setval('product_variants_id_seq', COALESCE(MAX(id), 0) + 1, false) FROM product_variants;
        ALTER TABLE product_variants ALTER COLUMN id SET DEFAULT nextval('product_variants_id_seq');
    END IF;
END $$;
