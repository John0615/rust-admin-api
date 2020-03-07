use actix_service::Service;
use actix_web::{test, web, App, HttpResponse, http::StatusCode};

#[actix_rt::test]
async fn test_init_service() {
    let mut app = test::init_service(
        App::new()
            .service(web::resource("/graphql").route(web::post().to(|| async { HttpResponse::Ok() })))
    ).await;

    // Create request object
    let req = test::TestRequest::post().uri("/graphql").to_request();

    // Execute application
    let resp = app.call(req).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}
