use actix_web::{test, web, App, HttpResponse};
use crate::graphql;
use crate::database;
use crate::cli_args;


#[actix_rt::test]
async fn test_index_get() {
    let opt = {
      use structopt::StructOpt;
      cli_args::Opt::from_args()
    };
    let schema = std::sync::Arc::new(crate::graphql::schemas::root::create_schema());
    let mut app = test::init_service(
      App::new()
        .data(database::pool::establish_connection(opt.clone()))
        .data(schema.clone())
        .data(opt.clone())
        .configure(graphql::route)
    ).await;
    let req = test::TestRequest::post().uri("/graphql").to_request();
    let resp = test::read_response_json(&mut app, req).await;
    eprintln!("resp==result>>>>>>:{:?}", resp);

    assert_eq!(100, 200);
}
