mod common;

use common::runtime_test;
use reign_router::middleware::Runtime;

#[cfg(feature = "router-tide")]
#[tokio::test]
async fn test_tide() {
    let server = async {
        let mut app = tide::new();

        app.at("/").get(|_| async move { Ok("hello") });

        app.middleware(Runtime::default());

        app.listen("127.0.0.1:8080").await.unwrap()
    };

    tokio::select! {
        _ = server => {},
        _ = runtime_test() => {},
    }
}
