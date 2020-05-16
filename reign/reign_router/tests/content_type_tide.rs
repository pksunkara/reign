mod common;

use common::content_type_test;
use reign_router::middleware::ContentType;

#[cfg(feature = "router-tide")]
#[tokio::test]
async fn test_tide() {
    let server = async {
        let mut app = tide::new();

        app.at("/").post(|_| async move { Ok("hello") });

        app.middleware(ContentType::default().multipart());

        app.listen("127.0.0.1:8080").await.unwrap()
    };

    tokio::select! {
        _ = server => {},
        _ = content_type_test() => {},
    }
}
