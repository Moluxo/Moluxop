use diesel::dsl::exists;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::dsl::select;
use diesel::dsl::sum;
use diesel::sql_query;
use diesel::sql_types::Bool;
use diesel::prelude::QueryDsl;
use dotenvy::dotenv;
use std::env;
use crate::models::*;
use bigdecimal::BigDecimal;
use uuid::Uuid;
use diesel::result::Error;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn insertar_producto(nombre_prod:&str,desc:&str,prec:&BigDecimal,img:&str) -> bool{
    use crate::schema::productos;
    use crate::schema::productos::dsl::*;

    let connection = &mut establish_connection();
    
    /* 
    let result = wordpairs
        .first::<Tarea>(connection)
        .optional();
    */
    let existing_product = productos
                            .filter(nombre_producto.eq(nombre_prod))
                            .first::<Product>(connection)
                            .optional();

    match existing_product {
        Ok(Some(_)) => {
            println!("El producto ya existe!");
            false
        }
        Ok(None) => {
            println!("Insertando nuevo producto");
            let new_product = NewProduct {
                nombre_producto: &nombre_prod,
                descripcion: &desc,
                precio:&prec,
                imagen:&img,
            };
            match diesel::insert_into(productos::table)
                .values(&new_product)
                .execute(connection) {
                Ok(_) => {
                    println!("Producto guardado");
                    true},
                Err(_) => {
                    println!("Error guardando nuevo producto");
                    false
                }
            }
        }
        Err(_) => {
            println!("Error");
            false
        }
    }

}

pub fn add_carrito(id_us:&uuid::Uuid,id_prod:&uuid::Uuid,cant:&i32) -> bool{
    use crate::schema::carrito;
    use crate::schema::carrito::dsl::*;

    let connection = &mut establish_connection();
    
    let new_carrito = NuevoCarrito {
        id_usuario: id_us,
        id_producto: id_prod,
        cantidad: cant,
    };
    match diesel::insert_into(carrito::table)
        .values(&new_carrito)
        .execute(connection) {
        Ok(_) => {
            println!("Producto guardado en carrito");
            true},
        Err(_) => {
            println!("Error guardando producto en carrito");
            false
        }
    }
}

pub fn existe_en_carrito(id_u: &Uuid, id_p: &Uuid) -> Result<bool,Error> {
    use crate::schema::carrito::dsl::*;
    let connection = &mut establish_connection();
    let resultado = select(exists(carrito.filter(id_usuario.eq(id_u).and(id_producto.eq(id_p))))).get_result(connection);
    
    resultado
}

pub fn existe_en_carrito_user(id_u: &Uuid) -> Result<bool,Error> {
    use crate::schema::carrito::dsl::*;
    let connection = &mut establish_connection();
    let resultado = select(exists(carrito.filter(id_usuario.eq(id_u)))).get_result(connection);
    
    resultado
}

pub fn carrito_numero_productos(id_u: &Uuid) -> i64 {
    use crate::schema::carrito::dsl::*;
    let connection = &mut establish_connection();
    let resultado = carrito.filter(id_usuario.eq(id_u))
                           .select(sum(cantidad))
                           .first::<Option<i64>>(connection);
    
    match resultado {
        Ok(Some(cant)) => {
            print!("Cantidad en carrito: {}",cant.to_string());
            cant
        },
        Ok(None) => 0,
        Err(_) => 0,
    }
}



pub fn obtener_cantidad_carrito(id_u: &Uuid, id_p: &Uuid) -> i32 {
    use crate::schema::carrito::dsl::*;

    let connection = &mut establish_connection();
    let resultado = carrito.filter(id_usuario.eq(id_u).and(id_producto.eq(id_p)))
                           .select(cantidad)
                           .first::<i32>(connection);
    
    match resultado {
        Ok(cant) => {
            println!("Cantidad en carrito: {}",cant.to_string());
            cant
        },
        Err(_) => 0,
    }
}

pub fn decrementar_cantidad_carrito(id_u: &Uuid, id_p: &Uuid) -> bool {
    use crate::schema::carrito::dsl::*;

    let connection = &mut establish_connection();

    match diesel::update(carrito.filter(id_usuario.eq(id_u).and(id_producto.eq(id_p))))
        .set(cantidad.eq(cantidad - 1))
        .execute(connection) {
        Ok(_) => {
            println!("Cantidad decrementada en carrito");
            true
        },
        Err(_) => {
            println!("Error decrementando cantidad en carrito");
            false
        }
    }
}

pub fn incrementar_cantidad_carrito(id_u: &Uuid, id_p: &Uuid) -> bool {
    use crate::schema::carrito::dsl::*;

    let connection = &mut establish_connection();

    match diesel::update(carrito.filter(id_usuario.eq(id_u).and(id_producto.eq(id_p))))
        .set(cantidad.eq(cantidad + 1))
        .execute(connection) {
        Ok(_) => {
            println!("Cantidad incrementada en carrito");
            true
        },
        Err(_) => {
            println!("Error incrementando cantidad en carrito");
            false
        }
    }
}

pub fn delete_carrito(id_u: &Uuid, id_p: &Uuid) -> bool {
    use crate::schema::carrito::dsl::*;
    let connection = &mut establish_connection();
    if(obtener_cantidad_carrito(id_u, id_p)<1){
        println!("No hay productos en el carrito, usuario o producto no existen");
        return false;
    }
    match diesel::delete(carrito.filter(id_usuario.eq(id_u).and(id_producto.eq(id_p))))
        .execute(connection) {
        Ok(_) => {
            println!("Producto eliminado del carrito");
            true
        },
        Err(_) => {
            println!("Error eliminando producto del carrito");
            false
        }
    }
}



pub fn list_productos(){
    use crate::schema::productos::dsl::*;

    let connection = &mut establish_connection();
    let results = productos
        .load::<Product>(connection)
        .expect("Error loading products");

    println!("Displaying {} products", results.len());
    for product in results {
        println!("{}", product.nombre_producto);
        println!("----------\n");
        println!("{}", product.descripcion.unwrap());
        println!("----------\n");
        println!("{}", product.precio);
        println!("----------\n");
        println!("{}", product.imagen.unwrap());
        println!("----------\n");
    }
}

pub fn tiene_productos() -> (bool,Vec<Product>){
    use crate::schema::productos::dsl::*;

    let connection = &mut establish_connection();
    let results = productos
        .select(Product::as_select())
        .load(connection)
        .expect("Error loading posts");

    if results.len() > 0 {
        return (true,results);
    }else{
        return (false,results);
    }
}

pub fn list_carrito(id_u: &Uuid) -> Vec<(ElemCarrito,Product)>{
    use crate::schema::carrito::dsl::*;
    use crate::schema::productos::dsl::*;

    let connection = &mut establish_connection();
    let results = carrito
        .inner_join(productos)
        .filter(id_usuario.eq(id_u))
        .select((ElemCarrito::as_select(),Product::as_select()))
        .load::<(ElemCarrito,Product)>(connection)
        .expect("Error loading carrito");

    //ahora mostramos los productos
    //println!("Displaying {} products in carrito", results.len());
    /*for (elem,product) in &results {
        println!("Nombre: {}", product.nombre_producto);
        println!("Descripcion: {}", product.descripcion.unwrap());
        println!("Precio: {}", product.precio);
        println!("Imagen: {}", product.imagen.unwrap());
        println!("Cantidad: {}", elem.cantidad);
        println!("----------\n");
    }*/
    results
}

pub fn total_carrito(id_u: &Uuid) -> BigDecimal {
    use crate::schema::carrito::dsl::*;
    use crate::schema::productos::dsl::*;

    let connection = &mut establish_connection();
    let results = carrito
        .inner_join(productos)
        .filter(id_usuario.eq(id_u))
        .select((precio,cantidad))
        .load::<(BigDecimal,i32)>(connection)
        .expect("Error loading carrito");

    let mut total: BigDecimal = BigDecimal::from(0);

    for (pre,cant) in &results {
        total = total + (pre * cant);
        
    }
    total
}