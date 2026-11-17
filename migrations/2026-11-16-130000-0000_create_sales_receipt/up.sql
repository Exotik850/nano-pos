CREATE TABLE sales_receipt (
    id SERIAL PRIMARY KEY,
    customer_id INTEGER NOT NULL REFERENCES contact(id),
    user_id INTEGER NOT NULL REFERENCES users(id),
    doc_number VARCHAR(255) NOT NULL,
    description VARCHAR(3000),
    sales_receipt_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE sales_receipt_product (
    id SERIAL PRIMARY KEY,
    sales_receipt_id INTEGER NOT NULL REFERENCES sales_receipt(id),
    product_id INTEGER NOT NULL REFERENCES product(id),
    quantity INTEGER NOT NULL,
    price INTEGER,
    tax_rate DOUBLE PRECISION,
    description VARCHAR(3000)
);
