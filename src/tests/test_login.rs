use actix_web::{test, App, web};
use crate::database;
use crate::cli_args;
use crate::graphql::handler;


#[actix_rt::test]
async fn test_index_get() {
    let opt = {
      use structopt::StructOpt;
      cli_args::Opt::from_args()
    };
    let schema = std::sync::Arc::new(crate::graphql::schemas::root::create_schema());

    let mut srv = test::start(|| App::new()
                        .data(database::pool::establish_connection(opt.clone()))
                        .data(schema.clone())
                        .data(opt.clone())
                        .service(web::resource("/graphql").route(web::post().to(handler::graphql)))
    );

    let req = srv.post("/graphql");
    let response = req.send().await.unwrap();
    // eprintln!("resp==result>>>>>>:{:?}", resp.data);
    // assert_eq!(100, 200);
    assert!(response.status().is_success());
}
