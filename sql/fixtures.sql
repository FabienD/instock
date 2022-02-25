INSERT INTO  instock.brand (name, description) 
VALUES
    ('Sony', null),
    ('Microsoft', null),
    ('Nintendo', null)
;

INSERT INTO  instock.product (name, description, upc, brand_id) 
VALUES
    ('Sony PlayStation 5 Édition Standard', null, '808223051645', 1),
    ('Sony PlayStation 5 Édition Digital', null, '808223010987', 1),
    ('XBox serie X', null, '745808009755', 2),
    ('XBox serie S', null, '840262756396', 2),
    ('Nintendo Switch Oled Joy Blanc', null, '045496883386', 3),
    ('Nintendo Switch Oled Joy Bleu/Rouge', null, '045496883409', 3)
;

INSERT INTO  instock.merchant_product (product_id, url, merchant, tracked) 
VALUES
    (1, 'https://www.amazon.fr/PlayStation-%C3%89dition-Standard-DualSense-Couleur/dp/B08H93ZRK9', 'AmazonFr', true),
    (2, 'https://www.amazon.fr/PlayStation-Digital-Manette-DualSense-Couleur/dp/B08H98GVK8', 'AmazonFr', true),
    (3, 'https://www.amazon.fr/Xbox-X-plus-puissante/dp/B08H93ZRLL', 'AmazonFr', true),
    (4, 'https://www.amazon.fr/Xbox-Console-Next-Gen-compacte-digitale/dp/B087VM5XC6', 'AmazonFr', true),
    (5, 'https://www.amazon.fr/Console-Nintendo-dAccueil-Manettes-Blanches/dp/B098RJXBTY', 'AmazonFr', true),
    (6, 'https://www.amazon.fr/Console-Nintendo-Switch-Mod%C3%A8le-Manettes/dp/B098BYN3X3', 'AmazonFr', true)
;
