// @generated automatically by Diesel CLI.

diesel::table! {
    carrito (clave) {
        clave -> Int4,
        id_usuario -> Uuid,
        id_producto -> Uuid,
        cantidad -> Int4,
    }
}

diesel::table! {
    productos (id_producto) {
        id_producto -> Uuid,
        nombre_producto -> Varchar,
        descripcion -> Nullable<Text>,
        precio -> Numeric,
        #[max_length = 255]
        imagen -> Nullable<Varchar>,
    }
}

diesel::joinable!(carrito -> productos (id_producto));

diesel::allow_tables_to_appear_in_same_query!(
    carrito,
    productos,
);
