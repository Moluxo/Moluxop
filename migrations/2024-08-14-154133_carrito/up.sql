-- Your SQL goes here
CREATE TABLE productos (
  id_producto UUID PRIMARY KEY DEFAULT gen_random_uuid(),
  nombre_producto VARCHAR NOT NULL,
  descripcion TEXT,
  precio NUMERIC(10,2) NOT NULL,
  imagen VARCHAR(255)
);


CREATE TABLE carrito (
    clave SERIAL PRIMARY KEY,
    id_usuario UUID NOT NULL, 
    id_producto UUID NOT NULL REFERENCES productos(id_producto),
    cantidad INT NOT NULL
);