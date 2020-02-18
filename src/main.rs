#[macro_use]
extern crate diesel;
extern crate serde_json;

mod route;
mod user;
mod graphql;
mod cli_args;
mod database;
mod errors;
mod models;
mod schema;
use actix_web::{ App, HttpServer };



#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    // 加载环境变量
    dotenv::dotenv().ok();

    let opt = {
        use structopt::StructOpt;
        cli_args::Opt::from_args()
    };

    let port = opt.port;
    eprintln!("Listening on 0.0.0.0:{}", port);
    HttpServer::new(move || {
        App::new()
            .data(database::pool::establish_connection(opt.clone()))
            .data(opt.clone())
            .configure(route::index)
            .configure(graphql::route)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
