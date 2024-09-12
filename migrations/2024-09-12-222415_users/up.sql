-- Your SQL goes here

CREATE TABLE "users"(
	"id" INT4 NOT NULL PRIMARY KEY,
	"username" TEXT NOT NULL,
	"email" TEXT NOT NULL,
	"password" TEXT NOT NULL,
	"created_at" TIMESTAMP NOT NULL,
	"updated_at" TIMESTAMP NOT NULL
);

