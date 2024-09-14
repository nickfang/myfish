-- Your SQL goes here

CREATE TABLE transactions (
  id SERIAL PRIMARY KEY,
  date DATE NOT NULL,
  name VARCHAR NOT NULL,
  amount integer NOT NULL,
  transaction_type VARCHAR NOT NULL,
  category VARCHAR NOT NULL,
  description VARCHAR(255) NOT NULL,
  note VARCHAR,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
)