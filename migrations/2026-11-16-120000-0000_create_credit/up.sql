CREATE TABLE credit (
    id SERIAL PRIMARY KEY,
    customer_id INTEGER NOT NULL REFERENCES contact(id),
    user_id INTEGER NOT NULL REFERENCES users(id),
    doc_number VARCHAR(255) NOT NULL,
    description VARCHAR(3000),
    credit_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE credit_product (
    id SERIAL PRIMARY KEY,
    credit_id INTEGER NOT NULL REFERENCES credit(id),
    product_id INTEGER NOT NULL REFERENCES product(id),
    quantity INTEGER NOT NULL,
    price INTEGER,
    tax_rate DOUBLE PRECISION,
    description VARCHAR(3000)
);
