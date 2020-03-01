use actix_web::{test, web, App, HttpResponse};
use crate::graphql;


#[actix_rt::test]
async fn test_index_get() {
    let mut app = test::init_service(App::new().configure(graphql::route)).await;
    let req = test::TestRequest::post().uri("/graphql").to_request();
    let resp = test::call_service(&mut app, req).await;
    eprintln!("Listening on 0.0.0.0:{:?}", resp.response());

    assert_eq!(resp.status(), 200);
}
