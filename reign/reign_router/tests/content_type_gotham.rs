mod common;

use common::content_type_test;
use reign_router::middleware::ContentType;

#[cfg(feature = "router-gotham")]
#[tokio::test]
async fn test_gotham() {
    use gotham::{
        init_server,
        pipeline::{new_pipeline, single::single_pipeline},
        router::builder::{build_router, DefineSingleRoute, DrawRoutes},
        state::State,
    };

    let server = async {
        fn hello(state: State) -> (State, &'static str) {
            (state, "hello")
        }

        let (chain, pipelines) = single_pipeline(
            new_pipeline()
                .add(ContentType::default().multipart())
                .build(),
        );

        let router = build_router(chain, pipelines, |route| {
            route.post("/").to(hello);
        });

        init_server("127.0.0.1:8080", router).await.unwrap()
    };

    tokio::select! {
        _ = server => {},
        _ = content_type_test() => {},
    }
}
