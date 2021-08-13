CREATE TABLE products (
    id SERIAL PRIMARY KEY,
    price FLOAT NOT NULL,
    title VARCHAR NOT NULL,
    body TEXT NOT NULL,
    published BOOLEAN NOT NULL DEFAULT 'f'
)
