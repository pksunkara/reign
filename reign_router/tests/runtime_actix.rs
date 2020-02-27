mod common;

use common::runtime_test;
use reign_router::middleware::Runtime;

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
                .wrap(Runtime::default())
                .route("/", web::get().to(hello))
        })
        .bind("127.0.0.1:8080")
        .unwrap()
        .run()
        .await
        .unwrap()
    });

    runtime_test().await;
}
