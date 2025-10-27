DROP TABLE IF EXISTS products;
DROP TABLE IF EXISTS sold;

CREATE TABLE products (
    id INTEGER PRIMARY KEY,
    stock INTEGER
);

CREATE TABLE sold (
    id INTEGER PRIMARY KEY,
    product_id INTEGER,
    quantity INTEGER
);

INSERT INTO products (id, stock) VALUES (1, 10);

BEGIN TRANSACTION;
SAVEPOINT start_sale;

UPDATE products
SET stock = stock - 5
WHERE id = 1;

INSERT INTO sold (product_id, quantity)
VALUES (1, 5);

END TRANSACTION;

SELECT * FROM sold;
SELECT * FROM products;