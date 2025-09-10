DROP TABLE IF EXISTS customer;
DROP TABLE IF EXISTS transactions;

CREATE TABLE customer (
    id INTEGER PRIMARY KEY,
    first_name,
    last_name,
    email,
    address
);

CREATE TABLE transactions (
    transaction_id,
    customer_id,
    price,
    purchase
);

INSERT INTO customer (first_name, last_name, email, address) VALUES
('John', 'Doe', 'john.doe@email.com', '123 Main St, New York, NY'),
('Jane', 'Smith', 'jane.smith@email.com', '456 Oak Ave, Los Angeles, CA'),
('Mike', 'Johnson', 'mike.johnson@email.com', '789 Pine Rd, Chicago, IL'),
('Sarah', 'Wilson', 'sarah.wilson@email.com', '321 Elm St, Houston, TX'),
('David', 'Brown', 'david.brown@email.com', '654 Maple Dr, Phoenix, AZ'),
('Emily', 'Davis', 'emily.davis@email.com', '555 Cedar Ln, Philadelphia, PA'),
('Robert', 'Miller', 'robert.miller@email.com', '888 Birch St, San Antonio, TX');

INSERT INTO transactions (customer_id, price, purchase) VALUES
(1, 29.99, 'T-Shirt'),
(1, 15.50, 'Coffee Mug'),
(2, 89.99, 'Headphones'),
(3, 45.75, 'Book Collection'),
(2, 12.99, 'Phone Case'),
(4, 199.99, 'Smart Watch'),
(5, 24.50, 'Water Bottle'),
(2, 9.99, 'Stickers'),
(3, 79.99, 'Wireless Mouse'),
(5, 34.99, 'Backpack');

SELECT * FROM customer;
SELECT * FROM transactions;

SELECT *
    FROM customer
    JOIN transactions ON customer.id = transactions.customer_id
    ;
    
SELECT *
    FROM customer
    LEFT JOIN transactions ON customer.id = transactions.customer_id
    ;