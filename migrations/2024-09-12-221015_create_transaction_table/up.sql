-- Your SQL goes here

CREATE TABLE transactions (
  id SERIAL PRIMARY KEY,
  date DATE NOT NULL,
  name TEXT NOT NULL,
  amount NUMERIC(10, 2) NOT NULL,
  transaction_type TEXT NOT NULL,
  category TEXT NOT NULL,
  description VARCHAR(255) NOT NULL,
  note TEXT,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL,
  updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
)