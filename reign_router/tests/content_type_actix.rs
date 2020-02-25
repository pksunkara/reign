mod common;

use common::content_type_test;
use reign_router::middleware::ContentType;

#[cfg(feature = "router-actix")]
#[actix_rt::test]
async fn test_actix() {
    use actix_web::{web, App, HttpRequest, HttpServer, Responder};

    actix_rt::spawn(async {
        async fn hello(_: HttpRequest) -> impl Responder {
            "hello"
        }

        HttpServer::new(|| {
            App::new()
                .wrap(ContentType::default().multipart())
                .route("/", web::post().to(hello))
        })
        .bind("127.0.0.1:8080")
        .unwrap()
        .run()
        .await
        .unwrap()
    });

    content_type_test().await;
}
