use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_files as fs;
use uuid::Uuid;

mod funciones;
mod database;
mod models;
mod schema;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    let id_base = Uuid::parse_str("95022733-f013-301a-0ada-abc18f151006").unwrap();
    //database::list_tareas(); //print de la base de datos

    HttpServer::new(|| {
        App::new()
            .service(funciones::index)
            .service(funciones::carrito)
            .service(funciones::add_carrito)
            .service(funciones::quitar_producto)
            .service(funciones::add_producto)
            .service(funciones::actualizar_precio)
            .service(funciones::actualizar_num_carrito)
            .service(funciones::reset_carrito_pop)
            .service(funciones::quitar_carrito)
            .service(fs::Files::new("/assets", "assets").show_files_listing())
            
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
