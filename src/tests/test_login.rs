use actix_web::{test, web, App};
use crate::modules::user::{aa, bb};

#[actix_rt::test]
async fn test_index_get() {
    let mut app = test::init_service(App::new().route("/", web::get().to(aa))).await;
    let req = test::TestRequest::with_header("content-type", "text/plain").to_request();
    let resp = test::call_service(&mut app, req).await;
    assert!(resp.status().is_success());
}

#[actix_rt::test]
async fn test_index_post() {
    let mut app = test::init_service(App::new().route("/again", web::get().to(bb))).await;
    let req = test::TestRequest::with_header("content-type", "text/plain").to_request();
    let resp = test::call_service(&mut app, req).await;
    assert!(resp.status().is_client_error());
}