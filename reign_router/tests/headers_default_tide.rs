mod common;

use common::headers_default_test;
use reign_router::middleware::HeadersDefault;

#[cfg(feature = "router-tide")]
#[tokio::test]
async fn test_tide() {
    let server = async {
        let mut app = tide::new();

        app.at("/").get(|_| async move { "hello" });

        app.middleware(HeadersDefault::new().add("x-version", "1.0"));

        app.listen("127.0.0.1:8080").await.unwrap()
    };

    tokio::select! {
        _ = server => {},
        _ = headers_default_test() => {},
    }
}
