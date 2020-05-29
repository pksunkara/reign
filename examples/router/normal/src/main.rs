#![feature(proc_macro_hygiene)]

use reign::{
    prelude::*,
    router::{
        middleware::{HeadersDefault, Runtime, },
        router::{serve, Router, Pipe},
    },
};

fn router(r: &mut Router) {

    r.pipe(
        Pipe::new("common")
            .and(HeadersDefault::empty().add("x-powered-by", "reign"))
    );
    r.pipe(Pipe::new("app").and(HeadersDefault::empty().add("x-content-type-options", "nosniff")));
    r.pipe(Pipe::new("timer").and(Runtime::default()));
    r.pipe(
        Pipe::new("api")
            .and(HeadersDefault::empty().add("x-version", "1.0"))
            .and(HeadersDefault::empty().add("content-type", "application/json"))
    );

    // scope!("", [common, app], {
    //     to!(get, "str", str_);
    //     to!(get, "string", string);
    //     to!(get, "response", response);

    //     to!(get, "error", error);

    //     to!(post, "post", post);
    //     to!(put, "put", put);
    //     to!(patch, "patch", patch);
    //     to!(delete, "delete", delete);

    //     to!([post, put], "multi_methods", multi_methods);

    //     scope!("scope_static", {
    //         get!("", scope_static);
    //     });

    //     scope!("pipe", [timer], {
    //         get!("", pipe);
    //     });

    //     scope!("pipe_empty", [], {
    //         get!("", pipe_empty);
    //     });

    //     get!("param" / id, param);
    //     get!("param_optional" / id: Option<String>, param_optional);

    //     get!("param_regex" / id @ "[0-9]+", param_regex);
    //     get!("param_optional_regex" / id: Option<String> @ "[0-9]+", param_optional_regex);

    //     get!("param_glob" / id: Vec<String>, param_glob);
    //     get!(
    //         "param_optional_glob" / id: Option<Vec<String>>,
    //         param_optional_glob
    //     );

    //     get!(
    //         "param_glob_middle" / id: Vec<String> / "foo",
    //         param_glob_middle
    //     );
    //     get!(
    //         "param_optional_glob_middle" / id: Option<Vec<String>> / "foo",
    //         param_optional_glob_middle
    //     );

    //     scope!("scope_param" / id, {
    //         get!("bar", scope_param);
    //     });
    //     scope!("scope_param_optional" / id: Option<String>, {
    //         get!("bar", scope_param_optional);
    //     });

    //     scope!("scope_param_regex" / id @ "[0-9]+", {
    //         get!("bar", scope_param_regex);
    //     });
    //     scope!("scope_param_optional_regex" / id: Option<String> @ "[0-9]+", {
    //         get!("bar", scope_param_optional_regex);
    //     });

    //     scope!("scope_param_glob" / id: Vec<String>, {
    //         get!("bar", scope_param_glob);
    //     });
    //     scope!("scope_param_optional_glob" / id: Option<Vec<String>>, {
    //         get!("bar", scope_param_optional_glob);
    //     });

    //     scope!("scope_param_glob_middle" / id: Vec<String> / "foo", {
    //         get!("bar", scope_param_glob_middle);
    //     });
    //     scope!(
    //         "scope_param_optional_glob_middle" / id: Option<Vec<String>> / "foo",
    //         {
    //             get!("bar", scope_param_optional_glob_middle);
    //         }
    //     );

    //     scope!("nested_scope" / foo, {
    //         scope!("foo" / bar, {
    //             get!("bar", nested_scope);
    //         });
    //     });

    //     get!("multi_params" / foo / "foo" / bar, multi_params);
    //     get!(
    //         "multi_globs" / foo: Vec<String> / "foobar" / bar: Vec<String>,
    //         multi_globs
    //     );

    //     // route.scope("sibling_scope/higher", |route| {
    //     //     route.get("").to(sibling_scope_higher);
    //     // });
    //     // route.scope("sibling_scope", |route| {
    //     //     route.get("higher").to(sibling_scope_common_higher);
    //     //     route.get("lower").to(sibling_scope_common_lower);
    //     //     route.get("c").to(sibling_scope_common_c);
    //     // });
    //     // route.scope("sibling_scope/lower", |route| {
    //     //     route.get("").to(sibling_scope_lower);
    //     // });
    // });
}

async fn server() {
    serve("127.0.0.1:8080", router).await.unwrap()
}

#[tokio::main]
async fn main() {
    server().await
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use test_examples::router::{test, StatusCode};
    use tokio::{select, time::delay_for};

    #[tokio::test]
    async fn test_server() {
        let client = async {
            delay_for(Duration::from_millis(100)).await;
            test(StatusCode::METHOD_NOT_ALLOWED).await
        };

        select! {
            _ =  server() => {}
            _ = client => {}
        }
    }
}
