use crate::schema::{productos, carrito};
use diesel::prelude::*;
use bigdecimal::BigDecimal;



#[derive(Insertable)]
#[diesel(table_name = productos)]
pub struct NewProduct<'a> {
    pub nombre_producto: &'a str,
    pub descripcion: &'a str,
    pub precio: &'a BigDecimal,
    pub imagen: &'a str,
}

#[derive(Insertable)]
#[diesel(table_name = carrito)]
pub struct NuevoCarrito<'a> {
    pub id_usuario: &'a uuid::Uuid,
    pub id_producto: &'a uuid::Uuid,
    pub cantidad: &'a i32,
}


#[derive(Queryable, Selectable, Identifiable)]
#[diesel(table_name = crate::schema::productos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(id_producto))]
pub struct Product {
    pub id_producto: uuid::Uuid,
    pub nombre_producto: String,
    pub descripcion: Option<String>,
    pub precio: BigDecimal,
    pub imagen: Option<String>,
}

#[derive(Queryable, Selectable, Identifiable, Associations)]
#[diesel(belongs_to(Product, foreign_key = id_producto))]
#[diesel(table_name = crate::schema::carrito)]
#[diesel(primary_key(clave))]
pub struct ElemCarrito {
    pub clave: i32,
    pub id_usuario: uuid::Uuid,
    pub id_producto: uuid::Uuid,
    pub cantidad: i32,
}