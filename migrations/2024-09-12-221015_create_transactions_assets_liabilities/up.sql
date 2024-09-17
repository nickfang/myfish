-- Your SQL goes here

CREATE TABLE "users" (
  "id" uuid PRIMARY KEY NOT NULL,
  "username" varchar NOT NULL,
  "email" varchar NOT NULL,
  "phone" varchar NOT NULL,
  "password" varchar NOT NULL,
  "created_at" timestamp DEFAULT (NOW()) NOT NULL,
  "updated_at" timestamp DEFAULT (NOW()) NOT NULL
);

CREATE TABLE "transactions" (
  "id" uuid PRIMARY KEY NOT NULL,
  "date" date NOT NULL,
  "name" varchar NOT NULL,
  "amount" integer DEFAULT 0,
  "transaction_type" varchar NOT NULL,
  "category" varchar NOT NULL,
  "description" text NOT NULL,
  "note" varchar,
  "user_id" uuid NOT NULL,
  "asset_id" uuid,
  "liability_id" uuid,
  "created_at" timestamp DEFAULT (Now()) NOT NULL,
  "updated_at" timestamp DEFAULT (Now()) NOT NULL
);

CREATE TABLE "assets" (
  "id" uuid PRIMARY KEY NOT NULL,
  "name" varchar NOT NULL,
  "value" integer DEFAULT 0 NOT NULL,
  "description" text,
  "longterm" bool DEFAULT false NOT NULL,
  "date_acquired" date DEFAULT (Now()),
  "date_divested" date,
  "user_id" uuid NOT NULL,
  "created_at" timestamp DEFAULT (Now()) NOT NULL,
  "updated_at" timestamp DEFAULT (Now()) NOT NULL
);

CREATE TABLE "liabilities" (
  "id" uuid PRIMARY KEY NOT NULL,
  "name" varchar NOT NULL,
  "description" text,
  "amount" integer DEFAULT 0,
  "payee" varchar,
  "payee_info" text,
  "longterm" bool DEFAULT false,
  "user_id" uuid NOT NULL,
  "created_at" timestamp DEFAULT (Now()) NOT NULL,
  "updated_at" timestamp DEFAULT (Now()) NOT NULL
);

ALTER TABLE "transactions" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id");

ALTER TABLE "assets" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id");

ALTER TABLE "liabilities" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("id");

ALTER TABLE "transactions" ADD FOREIGN KEY ("asset_id") REFERENCES "assets" ("id");

ALTER TABLE "transactions" ADD FOREIGN KEY ("liability_id") REFERENCES "liabilities" ("id");
