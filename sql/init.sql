-- Merchant plateform

CREATE TYPE merchant AS ENUM ('amazon_fr', 'fnac_fr', 'cdiscount_fr');

-- Product table

CREATE TABLE product IF NOT EXISTS  {
    ean text NOT NULL,
    url text UNIQUE NOT NULL,
    name text NOT NULL,
    description text,
    merchant merchant,
    tracked BOOLEAN DEFAULT false,
    PRIMARY KEY (ean, merchant)
}

