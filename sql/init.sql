-- Specific DB schema
CREATE SCHEMA instock
    AUTHORIZATION postgres;

-- Scraping methods enum type
CREATE TYPE instock.scraping_method AS ENUM ('library', 'browser');

-- Merchant plateform

CREATE TABLE IF NOT EXISTS instock.merchant (
    id UUID PRIMARY KEY,
    name text NOT NULL,
    scraping_elements jsonb NOT NULL,
    scraping_method instock.scraping_method NOT NULL DEFAULT('library')
);


-- Brand table

CREATE TABLE IF NOT EXISTS instock.brand (
    id UUID PRIMARY KEY,
    name text NOT NULL
);

-- Product table

CREATE TABLE IF NOT EXISTS instock.product (
    id UUID PRIMARY KEY,
    name text NOT NULL,
    description text,
    brand_id UUID NOT NULL,
    upc text,
    url text,
    image text
);

ALTER TABLE  instock.product
ADD CONSTRAINT product_brand_id_fk
    FOREIGN KEY (brand_id)
    REFERENCES  instock.brand(id)
    ON DELETE CASCADE;

-- Merchant Product table

CREATE TABLE IF NOT EXISTS instock.merchant_product (
    id UUID PRIMARY KEY,
    url text UNIQUE NOT NULL,
    product_id UUID NOT NULL,
    merchant_id UUID NOT NULL,
    tracked BOOLEAN DEFAULT false
);

ALTER TABLE  instock.merchant_product
ADD CONSTRAINT merchant_product_product_id_fk
    FOREIGN KEY (product_id)
    REFERENCES  instock.product(id)
    ON DELETE CASCADE;

-- Tracking table

CREATE TABLE IF NOT EXISTS  instock.tracking (
    merchant_product_id UUID,
    is_in_stock BOOLEAN NOT NULL DEFAULT false,
    tracked_at TIMESTAMP WITH TIME ZONE,
    PRIMARY KEY(merchant_product_id, tracked_at)
);

ALTER TABLE  instock.tracking
ADD CONSTRAINT tracking_merchant_product_id_fk
    FOREIGN KEY (merchant_product_id)
    REFERENCES  instock.merchant_product(id)
    ON DELETE CASCADE;

-- User table

CREATE TABLE IF NOT EXISTS  instock.user (
    id UUID PRIMARY KEY,
    email varchar(254) UNIQUE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE
);


-- User alerting table

CREATE TABLE IF NOT EXISTS instock.user_tracking (
    user_id UUID,
    merchant_product_id UUID,
    alerted_at TIMESTAMP WITH TIME ZONE,
    alert_count int DEFAULT 0,
    alert_count_max int DEFAULT 5,
    PRIMARY KEY(user_id, merchant_product_id)
);

ALTER TABLE  instock.user_tracking
ADD CONSTRAINT user_tracking_user_id_fk
    FOREIGN KEY (user_id)
    REFERENCES  instock.user(id)
    ON DELETE CASCADE;

ALTER TABLE  instock.user_tracking
ADD CONSTRAINT user_tracking_merchantproduct_id_fk
    FOREIGN KEY (merchant_product_id)
    REFERENCES  instock.merchant_product(id)
    ON DELETE CASCADE;