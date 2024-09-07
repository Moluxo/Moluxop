use actix_web::{get, post, web,delete, App, HttpResponse, HttpServer, Responder, http::StatusCode};
use bigdecimal::BigDecimal;
use lazy_static::lazy_static;
use tera::Tera;
use actix_files as fs;
use serde::Deserialize;
use serde::Serialize;
use std::{str::FromStr, sync::Mutex};
use crate::database::carrito_numero_productos;
use crate::database::{self, tiene_productos};
use uuid::Uuid;

lazy_static! { //al ser lazy static se ejecuta una sola vez ya que se reutiliza
    pub static ref TEMPLATES: Tera = {
        let source = "templates/**/*";
        let tera = Tera::new(&source).unwrap();
        tera
    };

    //momentaneo
    static ref ID_BASE:Uuid =Uuid::parse_str("95022733-f013-301a-0ada-abc18f151006").unwrap();
    static ref ID_BASE_JOKER:Uuid =Uuid::parse_str("95023733-f013-301a-0ada-abc18f151006").unwrap();

    //static ref TASK_COUNTER: Mutex<i32> = Mutex::new(0);
    //static ref NEXT_ID: AtomicI32 = AtomicI32::new(1);
}
#[derive(serde:: Serialize)]
struct Product {
    id: String,
    nombre: String,
    descripcion: String,
    precio: String,
    imagen: String,
}

#[derive(serde:: Serialize)]
struct ProductoCarrito {
    id: String,
    nombre: String,
    descripcion: String,
    precio: String,
    imagen: String,
    cantidad: String,
    num:String,
    disabled: String,
}


#[get("/")]
async fn index() -> impl Responder {
    add_products(); // se añaden productos a la base de datos
    database::list_productos();
    let mut products = Vec::new();
    let mut context1 = tera::Context::new();
    match tiene_productos(){
        (true,results) => {
            //los resultados los vot a poner en un nuevo objeto
            
            for product in results{
                let p = Product{
                    id: product.id_producto.to_string(),
                    nombre: product.nombre_producto,
                    descripcion: product.descripcion.unwrap(),
                    precio: product.precio.to_string(),
                    imagen: product.imagen.unwrap(),
                };
                products.push(p);
            }
            

            context1.insert("products",&products);
        },
        (false,_) => {
            context1.insert("lista_productos",&products);
        }
    }
    //ver si hay contenido en el carrito
    match database::existe_en_carrito_user(&ID_BASE){
        Ok(true) => {
            context1.insert("num_carrito", &carrito_numero_productos(&ID_BASE));
        },
        Ok(false) => {
            context1.insert("num_carrito", &0);
        },
        Err(_) => {
            println!("Error al buscar en carrito");
        }
    }


    let page_content: String = TEMPLATES.render("productos.html", &context1).unwrap();
    //print!("{}",page_content);
    HttpResponse::Ok().body(page_content)
}

#[get("/carrito")]
async fn carrito() -> impl Responder {
    let mut context = tera::Context::new();

    database::list_carrito(&ID_BASE);

    match database::existe_en_carrito_user(&ID_BASE){
        Ok(true) => {
            context.insert("num_carrito", &carrito_numero_productos(&ID_BASE));
        },
        Ok(false) => {
            context.insert("num_carrito", &0);
        },
        Err(_) => {
            println!("Error al buscar en carrito");
        }
    }
    
    match database::list_carrito(&ID_BASE){
        vector => {
            let mut products = Vec::new();
            let mut contador = 0;
            for (elem_carri,prod) in vector{
                let p = ProductoCarrito{
                    id: prod.id_producto.to_string(),
                    nombre: prod.nombre_producto,
                    descripcion: prod.descripcion.unwrap(),
                    precio: prod.precio.to_string(),
                    imagen: prod.imagen.unwrap(),
                    cantidad: elem_carri.cantidad.to_string(),
                    //contador de productos
                    num: contador.to_string(),
                    disabled: if elem_carri.cantidad == 1 { "disabled".to_string() } else { "".to_string() },
                };
                products.push(p);
                contador += 1;
            }
            context.insert("products",&products);
        }

    }

    context.insert("total", &database::total_carrito(&ID_BASE).to_string());

    let page_content = TEMPLATES.render("carrito.html", &context).unwrap();
    //println!("{}",page_content);
    HttpResponse::Ok().body(page_content)
}

#[get("/quitar-producto/{id}")]
async fn quitar_producto(path: web::Path<String>) -> impl Responder {
    let id_str = path.into_inner();
    let pid_uuid = Uuid::parse_str(&id_str).unwrap();
    let mut cantidad_carrito=-1;
    match database::existe_en_carrito(&ID_BASE, &pid_uuid){
        Ok(true) => {
            cantidad_carrito = database::obtener_cantidad_carrito(&ID_BASE, &pid_uuid);
            //tenemos que asegurarnos que la cantidad no sea 0
            if cantidad_carrito > 1{
                //si la cantidad es 1, eliminamos el producto
                if(database::decrementar_cantidad_carrito(&ID_BASE, &pid_uuid)){
                    cantidad_carrito -= 1;
                }
            //si existe en el carrito, decrementamos la cantidad
            }else{
                println!("No se puede quitar más productos");
            }
            
            println!("Cantidad en carrito: {}",cantidad_carrito);
        },
        Ok(false) => {
            //si no existe en el carrito, lo añadimos
            println!("No existe en el carrito");
        },
        Err(_) => {
            println!("Error al quitar producto del carrito");
        }
    }
    
    HttpResponse::Ok()
        .insert_header(("HX-Trigger","actualizar-num-carrito"))
        .append_header(("HX-Trigger","actualizar-precio"))
        .body(cantidad_carrito.to_string())
}

#[get("/add-producto/{id}")]
async fn add_producto(path: web::Path<String>) -> impl Responder {
    let id_str = path.into_inner();
    let pid_uuid = Uuid::parse_str(&id_str).unwrap();
    match database::existe_en_carrito(&ID_BASE, &pid_uuid){
        Ok(true) => {
            //si existe en el carrito, incrementamos la cantidad
            database::incrementar_cantidad_carrito(&ID_BASE, &pid_uuid);
        },
        Ok(false) => {
            //si no existe en el carrito, lo añadimos
            println!("No existe en el carrito");
        },
        Err(_) => {
            println!("Error al añadir producto al carrito");
        }
    }
    HttpResponse::Ok()
        .insert_header(("HX-Trigger","actualizar-num-carrito"))
        .append_header(("HX-Trigger","actualizar-precio"))
        .body(database::obtener_cantidad_carrito(&ID_BASE, &pid_uuid).to_string())
}


#[get("/add-carrito/{id}")]
async fn add_carrito(path: web::Path<String>) -> impl Responder {
    let id_str = path.into_inner();
    //vamos a ver si el producto existe en la base de datos del carrito
    //database::existe_en_carrito(id_u, id_p);
    let pid_uuid = Uuid::parse_str(&id_str).unwrap();

    match database::existe_en_carrito(&*ID_BASE, &pid_uuid){
        Ok(true) => {
            //si existe en el carrito, incrementamos la cantidad
            database::incrementar_cantidad_carrito(&*ID_BASE, &pid_uuid);
        },
        Ok(false) => {
            //si no existe en el carrito, lo añadimos
            database::add_carrito(&*ID_BASE, &pid_uuid,&1);
        },
        Err(_) => {
            println!("Error al añadir producto al carrito");
        }
    }
    let mut context = tera::Context::new();

    context.insert("titulo_aviso",  &format!("{} añadido al carrito",id_str));
    context.insert("mensaje_aviso", "Funciona!");
    let page_content = TEMPLATES.render("pop-carrito.html", &context).unwrap();
    HttpResponse::Ok()
        .insert_header(("HX-Trigger","actualizar-num-carrito"))
        .body(page_content)
}


#[get("/actualizar-precio")]
async fn actualizar_precio() -> impl Responder {
    HttpResponse::Ok().body(database::total_carrito(&ID_BASE).to_string()+" €")
}

#[get("/actualizar-num-carrito")]
async fn actualizar_num_carrito() -> impl Responder {
    HttpResponse::Ok().body(carrito_numero_productos(&ID_BASE).to_string())
}

#[get("/reset-carrito-pop/{id}")]
async fn reset_carrito_pop(path: web::Path<String>) -> impl Responder {
    let id_str = path.into_inner();
    let pid_uuid = Uuid::parse_str(&id_str).unwrap();

    let mut context = tera::Context::new();

    context.insert("id",  &id_str);
    let page_content = TEMPLATES.render("pop-eliminar.html", &context).unwrap();
    HttpResponse::Ok()
        .body(page_content)

}

#[delete("/quitar-carrito/{id}")]
async fn quitar_carrito(path: web::Path<String>) -> impl Responder {
    let id_str = path.into_inner();
    let id = Uuid::parse_str(&id_str).unwrap();
    if(database::delete_carrito(&ID_BASE, &id)){
        HttpResponse::Ok()
            .insert_header(("HX-Trigger","actualizar-num-carrito"))
            .append_header(("HX-Trigger","actualizar-precio"))
            .finish()
    }else{
        HttpResponse::new (StatusCode::NO_CONTENT) // no hace nada, se cancela la petición
    }
    
    
}


//funcion de añadir muchos productos
pub fn add_products(){
    let precio = BigDecimal::from_str("12.05").unwrap();
    let precio2 = BigDecimal::from_str("20.00").unwrap(); 
    let precio3 = BigDecimal::from_str("1.00").unwrap(); 
    let precio4 = BigDecimal::from_str("50.99").unwrap(); 
    let precio5 = BigDecimal::from_str("14.90").unwrap(); 
    let img = "assets/imgs/amarillo.jpg";
    let img2 = "assets/imgs/verde.jpg";
    let img3 = "assets/imgs/azul.jpg";
    let img4 = "assets/imgs/celeste.jpg";
    let img5 = "assets/imgs/deadpool-funko.jpg";

    database::insertar_producto("Producto1", "Es un buen producto", &precio, img);
    database::insertar_producto("Producto2", "Pedazo de producto", &precio2, img2);
    database::insertar_producto("Producto3", "Producto bastante interesante", &precio3, img3);
    database::insertar_producto("Producto4", "Producto categoría S", &precio4, img4);
    database::insertar_producto("DeadPool Figura", "Figura de Deadpool", &precio5, img5);
}