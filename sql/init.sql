-- Merchant plateform

CREATE TYPE merchant AS ENUM ('AmazonFr', 'FnacFr', 'CdiscountFr');

-- Product table

CREATE TABLE IF NOT EXISTS product (
    id serial PRIMARY KEY,
    url text UNIQUE NOT NULL,
    name text NOT NULL,
    description text,
    merchant merchant NOT NULL,
    upc text,
    tracked BOOLEAN DEFAULT false
)
