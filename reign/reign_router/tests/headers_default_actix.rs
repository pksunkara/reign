mod common;

use common::headers_default_test;
use reign_router::middleware::HeadersDefault;

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
                .wrap(HeadersDefault::empty().add("x-version", "1.0"))
                .route("/", web::get().to(hello))
        })
        .bind("127.0.0.1:8080")
        .unwrap()
        .run()
        .await
        .unwrap()
    });

    headers_default_test().await;
}
