-- Your SQL goes here
create table estimate (
  id SERIAL PRIMARY KEY,
  customer_id INT REFERENCES contact(id),
  user_id INT NOT NULL REFERENCES users(id),
  doc_number VARCHAR(255) NOT NULL,
  description VARCHAR(3000),
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

create table estimate_product (
  id SERIAL PRIMARY KEY,
  estimate_id INT NOT NULL REFERENCES estimate(id),
  product_id INT NOT NULL REFERENCES product(id),
  quantity INT NOT NULL,
  price INT,
  tax_rate DOUBLE PRECISION,
  description VARCHAR(3000)
);