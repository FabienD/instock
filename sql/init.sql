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
    scraping_method instock.scraping_method NOT NULL DEFAULT('browser')
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
    price VARCHAR(64) DEFAULT NULL,
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
    is_enabled BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT now()
);


-- User alerting table

CREATE TABLE IF NOT EXISTS instock.user_tracking (
    user_id UUID,
    product_id UUID,
    alerted_at TIMESTAMP WITH TIME ZONE,
    alert_count int DEFAULT 0,
    alert_count_max int DEFAULT 5,
    PRIMARY KEY(user_id, product_id)
);

ALTER TABLE  instock.user_tracking
ADD CONSTRAINT user_tracking_user_id_fk
    FOREIGN KEY (user_id)
    REFERENCES  instock.user(id)
    ON DELETE CASCADE;

ALTER TABLE  instock.user_tracking
ADD CONSTRAINT user_tracking_product_id_fk
    FOREIGN KEY (product_id)
    REFERENCES  instock.product(id)
    ON DELETE CASCADE;


-- Create Timescale DB  hyperscale table

SELECT create_hypertable('instock.tracking','tracked_at');

CREATE INDEX ix_product_tracked_at ON instock.tracking (merchant_product_id, tracked_at DESC);

-- Patch --

ALTER TABLE IF EXISTS instock.merchant_product
    ADD COLUMN affiliate_url text;

ALTER TABLE IF EXISTS instock.merchant_product
    ADD CONSTRAINT merchant_product_affiliate_url_key UNIQUE (affiliate_url);

ALTER TABLE IF EXISTS instock.user_tracking
    ADD COLUMN max_price numeric(8, 2);


CREATE TABLE IF NOT EXISTS  instock.user_auth_info (
    user_id UUID PRIMARY KEY,
    salt CHAR(32) NOT NULL,
    hash CHAR(32) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NULL
);

ALTER TABLE  instock.user_auth_info
ADD CONSTRAINT user_auth_info_user_id_fk
    FOREIGN KEY (user_id)
    REFERENCES  instock.user(id)
    ON DELETE CASCADE;

ALTER TABLE instock.user 
    ADD COLUMN link_id UUID DEFAULT gen_random_uuid();