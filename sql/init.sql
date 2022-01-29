-- Specific DB schema
CREATE SCHEMA instock
    AUTHORIZATION postgres;

-- Merchant plateform

CREATE TYPE instock.merchant AS ENUM ('AmazonFr', 'FnacFr', 'CdiscountFr');

-- Product table

CREATE TABLE IF NOT EXISTS instock.product (
    id serial PRIMARY KEY,
    url text UNIQUE NOT NULL,
    name text NOT NULL,
    description text,
    merchant instock.merchant NOT NULL,
    upc text,
    tracked BOOLEAN DEFAULT false
);

-- Tracking table

CREATE TABLE IF NOT EXISTS  instock.tracking (
    product_id int,
    is_in_stock BOOLEAN NOT NULL DEFAULT false,
    tracked_at TIMESTAMP WITH TIME ZONE,
    PRIMARY KEY(product_id, tracked_at)
);

ALTER TABLE  instock.tracking
ADD CONSTRAINT tracking_product_id_fk
    FOREIGN KEY (product_id)
    REFERENCES  instock.product(id)
    ON DELETE CASCADE;

-- User table

CREATE TABLE IF NOT EXISTS  instock.user (
    id serial PRIMARY KEY,
    email varchar(254) UNIQUE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE
);


-- User alerting table

CREATE TABLE IF NOT EXISTS instock.user_tracking (
    user_id int,
    product_id int,
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