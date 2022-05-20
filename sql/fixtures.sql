INSERT INTO  instock.merchant (id, name, scraping_elements) 
VALUES
    ('8ade4ef2-a946-4791-84c5-326362e00bfb', 'Amazon FR', '{"title": "#productTitle", "cart": "#add-to-cart-button"}'),
    ('b81da46a-be47-4049-b033-edc4ac67f8cf', 'Amazon DE', '{"title": "#productTitle", "cart": "#add-to-cart-button"}'),
    ('ef4381db-8140-4dfd-9ac0-4cc68188af88', 'Amazon IT', '{"title": "#productTitle", "cart": "#add-to-cart-button"}'),
    ('6f590e2a-2d7a-4084-a500-e91f8b8a38e8', 'Amazon ES', '{"title": "#productTitle", "cart": "#add-to-cart-button"}'),
    ('bf9b3fad-dbf4-4135-bdc5-1f88898acead', 'Amazon UK', '{"title": "#productTitle", "cart": "#add-to-cart-button"}'),
    ('a49db0c9-c1d5-4fbd-877e-ce21df3b3e53', 'Fnac FR', '{"title": "f-productHeader-Title", "cart": "f-buyBox-buttons js-ProductBuy-add"}'),
    ('4acdaed8-5764-4623-9789-1c787ab13a08', 'Micromania','{"title": "h1.pdp-product-name span", "cart": "button.add-to-cart-global"}'),
    ('1b87425a-4eb3-4e18-a493-29a3517ae3ca', 'Boulanger', '{"title": "h1.product-title__main", "cart": "button.js-add-to-cart-popin"}'),
    ('ed0638dc-0a8e-42f2-be59-a1766feefb17', 'CDiscount', '{"title": "h1.fpSOTitleName", "cart": "#fpSOTitleName"}')
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
    ('d8e19447-84bb-4462-aed2-e05e856b2f51', 'https://www.fnac.com/Console-Sony-PS5-Edition-Standard/a14119956/w-4', false, 'a49db0c9-c1d5-4fbd-877e-ce21df3b3e53','d695d36d-dd42-42da-8d18-7a96ac87fdaf'),
    ('fd6f3bbb-06c5-4634-b549-ac945b71a787', 'https://www.amazon.fr/PlayStation-Digital-Manette-DualSense-Couleur/dp/B08H98GVK8', true, '8ade4ef2-a946-4791-84c5-326362e00bfb','1b9398b1-f4f1-4b8f-b045-47d92e53127c'),
    ('786872f0-a12b-4f8c-8b1a-a7716a2e7412', 'https://www.fnac.com/Console-Sony-PS5-Edition-Digital/a16174014/w-4', false, 'a49db0c9-c1d5-4fbd-877e-ce21df3b3e53','1b9398b1-f4f1-4b8f-b045-47d92e53127c'),
    ('1b528d59-e234-40db-afcb-b43b2f3ff770', 'https://www.amazon.fr/Xbox-X-plus-puissante/dp/B08H93ZRLL', false, '8ade4ef2-a946-4791-84c5-326362e00bfb','ac510747-9fc7-48bf-9d6d-44f155ef22cd'),
    ('9a23c9c7-d35f-4583-82b2-aee6353d8abf', 'https://www.amazon.fr/Xbox-Console-Next-Gen-compacte-digitale/dp/B087VM5XC6', true, '8ade4ef2-a946-4791-84c5-326362e00bfb', 'beb02920-4436-4d9b-bcae-e92db6fe6704'),
    ('accec422-2a17-4a5e-923e-7782bb18c0b4', 'https://www.amazon.fr/Console-Nintendo-dAccueil-Manettes-Blanches/dp/B098RJXBTY', true,'8ade4ef2-a946-4791-84c5-326362e00bfb', 'eb67e6d6-80d6-4568-9d0e-d440d19eeb6b'),
    ('a7a1273d-0c5e-41ad-8bcb-86db66993961', 'https://www.amazon.fr/Console-Nintendo-Switch-Mod%C3%A8le-Manettes/dp/B098BYN3X3', true,'8ade4ef2-a946-4791-84c5-326362e00bfb', '335f0b53-29c0-4692-ba91-c034c58623d2'),
    ('51ae397c-328a-4827-97ae-a2853da5e08f', 'https://www.micromania.fr/playstation-5-105642.html', false, '4acdaed8-5764-4623-9789-1c787ab13a08','d695d36d-dd42-42da-8d18-7a96ac87fdaf'),
    ('2dd29b66-556e-41e1-89dd-caa76951fa08', 'https://www.micromania.fr/playstation-5-alldigital-106097.html', false, '4acdaed8-5764-4623-9789-1c787ab13a08','1b9398b1-f4f1-4b8f-b045-47d92e53127c'),
    ('0d32244b-70d4-4bcb-b43a-e0bdf4cdb13e', 'https://www.boulanger.com/ref/1147567', true, '1b87425a-4eb3-4e18-a493-29a3517ae3ca','d695d36d-dd42-42da-8d18-7a96ac87fdaf'),
    ('cac793ee-29c6-47e0-9f2a-fc3666c00bf6', 'https://www.boulanger.com/ref/1147568', true, '1b87425a-4eb3-4e18-a493-29a3517ae3ca','1b9398b1-f4f1-4b8f-b045-47d92e53127c'),
    ('403bfcfa-6b16-44ac-86a4-f74fbee6013b', 'https://www.cdiscount.com/jeux-pc-video-console/ps5/console-ps5-sony-ps5/f-10350-son3665540797413.html', false, 'ed0638dc-0a8e-42f2-be59-a1766feefb17', 'd695d36d-dd42-42da-8d18-7a96ac87fdaf'),
    ('6545da1b-9de4-4697-918c-545731b9ed86', 'https://www.cdiscount.com/jeux-pc-video-console/ps5/console-sony-ps5-digital-edition-playstation-5-8/f-1035001-son4948872415002.html', false, 'ed0638dc-0a8e-42f2-be59-a1766feefb17', '1b9398b1-f4f1-4b8f-b045-47d92e53127c'),
    ('1aa6a031-60a3-4404-8b1c-f338439aa348', 'https://www.amazon.de/dp/B08H93ZRK9', true, 'b81da46a-be47-4049-b033-edc4ac67f8cf', 'd695d36d-dd42-42da-8d18-7a96ac87fdaf'),
    ('525ac84a-04db-4737-aae3-61984fc0981b', 'https://www.amazon.de/-/en/191088/dp/B08H98GVK8', true, 'b81da46a-be47-4049-b033-edc4ac67f8cf', '1b9398b1-f4f1-4b8f-b045-47d92e53127c'),
    ('e322b725-d90c-43be-983c-c9ef376b361d', 'https://www.amazon.es/Sony-PlayStation-Consola-5/dp/B08H93ZRK9', true, '6f590e2a-2d7a-4084-a500-e91f8b8a38e8', 'd695d36d-dd42-42da-8d18-7a96ac87fdaf'),
    ('002cb0df-33ba-4a0d-a181-c18a037d785f', 'https://www.amazon.es/Sony-PlayStation-Consola-5-Digital/dp/B08H98GVK8', true, '6f590e2a-2d7a-4084-a500-e91f8b8a38e8', '1b9398b1-f4f1-4b8f-b045-47d92e53127c'),
    ('c1d9e3da-d385-4c5e-9b8c-cbbb64f34681', 'https://www.amazon.it/Playstation-Sony-PlayStation-5/dp/B08KKJ37F7', true, 'ef4381db-8140-4dfd-9ac0-4cc68188af88', 'd695d36d-dd42-42da-8d18-7a96ac87fdaf'),
    ('3e950554-5fbe-4393-bf03-14f25813f2c8', 'https://www.amazon.it/Sony-PlayStation-5-Digital-Edition/dp/B08KJF2D25', true, 'ef4381db-8140-4dfd-9ac0-4cc68188af88', '1b9398b1-f4f1-4b8f-b045-47d92e53127c'),
    ('7677cc3c-1a3d-412c-bf01-afccb51b0830', 'https://www.amazon.co.uk/PlayStation-9395003-5-Console/dp/B08H95Y452', true, 'bf9b3fad-dbf4-4135-bdc5-1f88898acead', 'd695d36d-dd42-42da-8d18-7a96ac87fdaf'),
    ('941c5c71-f53d-4c75-ab93-875960a45e99', 'https://www.amazon.co.uk/PlayStation-5-Digital-Edition-Console/dp/B08H97NYGP', true, 'bf9b3fad-dbf4-4135-bdc5-1f88898acead', '1b9398b1-f4f1-4b8f-b045-47d92e53127c')
;

INSERT INTO  instock.user (id, email, is_enabled) 
VALUES
    ('f4ea6562-4b8a-4e42-a4d4-f73250863031', 'user1@example.com', true),
    ('7c6caf13-8463-4c2b-8881-296af35c2b5d', 'user2@example.com', false)
;

INSERT INTO  instock.user_tracking (user_id, product_id) 
VALUES
    ('f4ea6562-4b8a-4e42-a4d4-f73250863031', 'd695d36d-dd42-42da-8d18-7a96ac87fdaf'),
    ('f4ea6562-4b8a-4e42-a4d4-f73250863031', '1b9398b1-f4f1-4b8f-b045-47d92e53127c'),
    ('f4ea6562-4b8a-4e42-a4d4-f73250863031', 'eb67e6d6-80d6-4568-9d0e-d440d19eeb6b'),
    ('7c6caf13-8463-4c2b-8881-296af35c2b5d', 'd695d36d-dd42-42da-8d18-7a96ac87fdaf')
;
