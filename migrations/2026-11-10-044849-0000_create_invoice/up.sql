-- Your SQL goes here
create table invoice (
  id SERIAL PRIMARY KEY,
  customer_id INT NOT NULL REFERENCES contact(id),
  user_id INT NOT NULL REFERENCES users(id),
  doc_number VARCHAR(255) NOT NULL,
  description TEXT,
  order_date TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  delivery_date TIMESTAMP,
  created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
  updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

create table invoice_product (
  invoice_id INT NOT NULL REFERENCES invoice(id),
  product_id INT NOT NULL REFERENCES product(id),
  quantity INT NOT NULL,
  price INT,
  tax_rate DOUBLE PRECISION,
  description TEXT,
  primary key (invoice_id, product_id)
);