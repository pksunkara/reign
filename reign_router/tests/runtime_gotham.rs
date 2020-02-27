mod common;

use common::runtime_test;
use reign_router::middleware::Runtime;

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
                .add(Runtime::default())
                .build(),
        );

        let router = build_router(chain, pipelines, |route| {
            route.get("/").to(hello);
        });

        init_server("127.0.0.1:8080", router).await.unwrap()
    };

    tokio::select! {
        _ = server => {},
        _ = runtime_test() => {},
    }
}
