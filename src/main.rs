#[macro_use]
extern crate diesel;
extern crate serde_json;


mod route;
mod modules;
mod graphql;
mod cli_args;
mod database;
mod errors;
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

    // Create Juniper schema
    let schema = std::sync::Arc::new(crate::graphql::schemas::root::create_schema());

    HttpServer::new(move || {
        App::new()
            .data(database::pool::establish_connection(opt.clone()))
            .data(schema.clone())
            .data(opt.clone())
            .configure(route::index)
            .configure(graphql::route)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
