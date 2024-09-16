-- Your SQL goes here

CREATE TABLE "users" (
  "id" uuid PRIMARY KEY,
  "username" varchar,
  "email" varchar,
  "phone" varchar,
  "created_at" timestamp DEFAULT (NOW()),
  "updated_at" timestamp DEFAULT (NOW())
);

CREATE TABLE "transactions" (
  "id" uuid PRIMARY KEY,
  "date" timestamp,
  "name" varchar,
  "amount" integer DEFAULT 0,
  "transaction_type" varchar,
  "category" varchar,
  "description" text,
  "note" varchar,
  "user_id" uuid,
  "asset_id" uuid,
  "liability_id" uuid,
  "created_at" timestamp DEFAULT (Now()),
  "updated_at" timestamp DEFAULT (Now())
);

CREATE TABLE "assets" (
  "id" uuid PRIMARY KEY,
  "name" varchar,
  "value" integer DEFAULT 0,
  "description" text,
  "longterm" bool DEFAULT false,
  "date_acquired" date DEFAULT (Now()),
  "date_divested" date,
  "user_id" uuid,
  "created_at" timestamp DEFAULT 'Now()',
  "updated_at" timestamp DEFAULT 'Now()'
);

CREATE TABLE "liabilities" (
  "id" uuid PRIMARY KEY,
  "name" varchar,
  "description" text,
  "amount" integer DEFAULT 0,
  "payee" varchar,
  "payee_info" text,
  "longterm" bool DEFAULT false,
  "user_id" uuid,
  "created_at" timestamp DEFAULT 'Now()',
  "updated_at" timestamp DEFAULT 'Now()'
);

ALTER TABLE "transactions" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id");

ALTER TABLE "assets" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id");

ALTER TABLE "liabilities" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id");

ALTER TABLE "transactions" ADD FOREIGN KEY ("asset_id") REFERENCES "assets" ("id");

ALTER TABLE "transactions" ADD FOREIGN KEY ("liability_id") REFERENCES "liabilities" ("id");
