INSERT INTO  instock.merchant (id, name, scraping_elements) 
VALUES
    ('8ade4ef2-a946-4791-84c5-326362e00bfb', 'Amazon FR', '{"title": "#productTitle", "cart": "#add-to-cart-button"}'),
    ('b81da46a-be47-4049-b033-edc4ac67f8cf', 'Amazon DE', '{"title": "#productTitle", "cart": "#add-to-cart-button"}'),
    ('ef4381db-8140-4dfd-9ac0-4cc68188af88', 'Amazon IT', '{"title": "#productTitle", "cart": "#add-to-cart-button"}'),
    ('6f590e2a-2d7a-4084-a500-e91f8b8a38e8', 'Amazon ES', '{"title": "#productTitle", "cart": "#add-to-cart-button"}'),
    ('bf9b3fad-dbf4-4135-bdc5-1f88898acead', 'Amazon UK', '{"title": "#productTitle", "cart": "#add-to-cart-button"}'),
    ('a49db0c9-c1d5-4fbd-877e-ce21df3b3e53', 'Fnac FR', '{"title": "f-productHeader-Title", "cart": "f-buyBox-buttons js-ProductBuy-add"}')
;

INSERT INTO  instock.brand (id, name) 
VALUES
    ('6c939be8-e6dd-41ba-8559-b606c3973d42', 'Sony'),
    ('a306182a-9d37-4f1b-a069-fd28e3004518', 'Microsoft'),
    ('0cb7b153-0c39-4f54-a937-1b106791ae0d', 'Nintendo')
;

INSERT INTO  instock.product (id, name, description, upc, brand_id) 
VALUES
    ('d695d36d-dd42-42da-8d18-7a96ac87fdaf', 'Sony PlayStation 5 Édition Standard', null, '808223051645', '6c939be8-e6dd-41ba-8559-b606c3973d42'),
    ('1b9398b1-f4f1-4b8f-b045-47d92e53127c', 'Sony PlayStation 5 Édition Digital', null, '808223010987', '6c939be8-e6dd-41ba-8559-b606c3973d42'),
    ('ac510747-9fc7-48bf-9d6d-44f155ef22cd', 'XBox serie X', null, '745808009755', 'a306182a-9d37-4f1b-a069-fd28e3004518'),
    ('beb02920-4436-4d9b-bcae-e92db6fe6704', 'XBox serie S', null, '840262756396', 'a306182a-9d37-4f1b-a069-fd28e3004518'),
    ('eb67e6d6-80d6-4568-9d0e-d440d19eeb6b', 'Nintendo Switch Oled Joy Blanc', null, '045496883386', '0cb7b153-0c39-4f54-a937-1b106791ae0d'),
    ('335f0b53-29c0-4692-ba91-c034c58623d2', 'Nintendo Switch Oled Joy Bleu/Rouge', null, '045496883409', '0cb7b153-0c39-4f54-a937-1b106791ae0d')
;

INSERT INTO  instock.merchant_product (id, url, tracked, merchant_id, product_id) 
VALUES
    ('0798f189-8a59-41b2-914f-8846555df4cc', 'https://www.amazon.fr/PlayStation-%C3%89dition-Standard-DualSense-Couleur/dp/B08H93ZRK9', true, '8ade4ef2-a946-4791-84c5-326362e00bfb','d695d36d-dd42-42da-8d18-7a96ac87fdaf'),
    ('d8e19447-84bb-4462-aed2-e05e856b2f51', 'https://www.fnac.com/Console-Sony-PS5-Edition-Standard/a14119956/w-4', true, 'a49db0c9-c1d5-4fbd-877e-ce21df3b3e53','d695d36d-dd42-42da-8d18-7a96ac87fdaf'),
    ('fd6f3bbb-06c5-4634-b549-ac945b71a787', 'https://www.amazon.fr/PlayStation-Digital-Manette-DualSense-Couleur/dp/B08H98GVK8', true, '8ade4ef2-a946-4791-84c5-326362e00bfb','1b9398b1-f4f1-4b8f-b045-47d92e53127c'),
    ('786872f0-a12b-4f8c-8b1a-a7716a2e7412', 'https://www.fnac.com/Console-Sony-PS5-Edition-Digital/a16174014/w-4', true, 'a49db0c9-c1d5-4fbd-877e-ce21df3b3e53','1b9398b1-f4f1-4b8f-b045-47d92e53127c'),
    ('1b528d59-e234-40db-afcb-b43b2f3ff770', 'https://www.amazon.fr/Xbox-X-plus-puissante/dp/B08H93ZRLL', true, '8ade4ef2-a946-4791-84c5-326362e00bfb','ac510747-9fc7-48bf-9d6d-44f155ef22cd'),
    ('9a23c9c7-d35f-4583-82b2-aee6353d8abf', 'https://www.amazon.fr/Xbox-Console-Next-Gen-compacte-digitale/dp/B087VM5XC6', true, '8ade4ef2-a946-4791-84c5-326362e00bfb', 'beb02920-4436-4d9b-bcae-e92db6fe6704'),
    ('accec422-2a17-4a5e-923e-7782bb18c0b4', 'https://www.amazon.fr/Console-Nintendo-dAccueil-Manettes-Blanches/dp/B098RJXBTY', true,'8ade4ef2-a946-4791-84c5-326362e00bfb', 'eb67e6d6-80d6-4568-9d0e-d440d19eeb6b'),
    ('a7a1273d-0c5e-41ad-8bcb-86db66993961', 'https://www.amazon.fr/Console-Nintendo-Switch-Mod%C3%A8le-Manettes/dp/B098BYN3X3', true,'8ade4ef2-a946-4791-84c5-326362e00bfb', '335f0b53-29c0-4692-ba91-c034c58623d2')
;
