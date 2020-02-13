mod route;
mod user;
mod graphql;

use actix_web::{ App, HttpServer };



#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!");

    HttpServer::new(|| {
        App::new()
            .configure(route::index)
            .configure(graphql::route)
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
