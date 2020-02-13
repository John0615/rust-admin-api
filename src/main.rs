mod route;
mod user;
mod graphql;
mod cli_args;

use actix_web::{ App, HttpServer };



#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    let opt = {
        use structopt::StructOpt;
        cli_args::Opt::from_args()
    };
    let port = opt.port;
    eprintln!("Listening on 0.0.0.0:{}", port);
    HttpServer::new(|| {
        App::new()
            .configure(route::index)
            .configure(graphql::route)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
