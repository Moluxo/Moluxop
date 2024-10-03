INSERT INTO product (id, name) VALUES
('P001', 'Laptop'),
('P002', 'Smartphone'),
('P003', 'Tablet');

INSERT INTO sku (id, product_id, quant, price) VALUES
('SKU001', 'P001', 100, 599),
('SKU002', 'P001', 50, 699),
('SKU003', 'P002', 200, 399),
('SKU004', 'P003', 80, 299);

INSERT INTO att (id, name) VALUES
('A001', 'Color'),
('A002', 'Storage'),
('A003', 'Brand');

INSERT INTO att_options (id, att_id, value) VALUES
('AO001', 'A001', 'Black'),
('AO002', 'A001', 'Silver'),
('AO003', 'A002', '128GB'),
('AO004', 'A002', '256GB'),
('AO005', 'A003', 'Apple'),
('AO006', 'A003', 'Samsung');

INSERT INTO att_options_sku (sku_id, att_options_id) VALUES
('SKU001', 'AO001'),
('SKU001', 'AO003'),
('SKU002', 'AO002'),
('SKU002', 'AO003'),
('SKU003', 'AO001'),
('SKU003', 'AO004'),
('SKU004', 'AO005'),
('SKU004', 'AO006');

select * from att_options_sku;


SELECT a.name AS atributo, ao.value AS valor, s.id as sku
FROM product p
JOIN sku s ON p.id = s.product_id
JOIN att_options_sku aos ON s.id = aos.sku_id
JOIN att_options ao ON aos.att_options_id = ao.id
JOIN att a ON ao.att_id = a.id
WHERE p.id = 'P001'
ORDER BY a.name;