CREATE TABLE users (
	id int8 NOT NULL GENERATED ALWAYS AS IDENTITY,
	"name" varchar NOT NULL
);

INSERT INTO users ("name") VALUES ('pet my cat');
