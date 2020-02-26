use actix_web::{test, web, App, HttpResponse};


#[actix_rt::test]
async fn test_index_get() {
    let mut app = test::init_service(App::new().service(web::resource("/graphql").to(|| async { HttpResponse::Ok() }))).await;
    let req = test::TestRequest::post().uri("/graphql").to_request();
    let resp = test::call_service(&mut app, req).await;
    assert!(resp.status().is_success());
}