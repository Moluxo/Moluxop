-- Your SQL goes here
CREATE TABLE productos (
  id_producto UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  nombre_producto VARCHAR NOT NULL,
  descripcion TEXT,
  precio NUMERIC(10,2) NOT NULL,
  imagen VARCHAR(255)
);