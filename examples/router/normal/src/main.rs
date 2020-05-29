#![feature(proc_macro_hygiene)]

use reign::{
    prelude::*,
    router::{
        middleware::{HeadersDefault, Runtime},
        router::{hyper::Method, serve, Pipe, Router, Scope, Path},
    },
};

fn router(r: &mut Router) {
    r.pipe(Pipe::new("common").and(HeadersDefault::empty().add("x-powered-by", "reign")));
    r.pipe(Pipe::new("app").and(HeadersDefault::empty().add("x-content-type-options", "nosniff")));
    r.pipe(Pipe::new("timer").and(Runtime::default()));
    r.pipe(
        Pipe::new("api")
            .and(HeadersDefault::empty().add("x-version", "1.0"))
            .and(HeadersDefault::empty().add("content-type", "application/json")),
    );

    r.scope(Scope::empty().through(&["common", "app"]).to(|r| {
        r.get(Path::new("str"), str_);
        r.get(Path::new("string"), string);
        r.get(Path::new("response"), response);

        r.get(Path::new("error"), error);

        r.post(Path::new("post"), post);
        r.put(Path::new("put"), put);
        r.patch(Path::new("patch"), patch);
        r.delete(Path::new("delete"), delete);

        r.any(
            &[Method::POST, Method::PUT],
            Path::new("multi_methods"),
            multi_methods,
        );

        r.scope(Scope::new(Path::new("scope_static")).to(|r| {
            r.get(Path::empty(), scope_static);
        }));

        r.scope(Scope::new(Path::new("pipe")).through(&["timer"]).to(|r| {
            r.get(Path::empty(), pipe);
        }));

        r.get(Path::new("param").param("id"), param);
        r.get(Path::new("param_opt").param_opt("id"), param_opt);

        r.get(
            Path::new("param_regex").param_regex("id", "[0-9]+"),
            param_regex,
        );
        r.get(
            Path::new("param_opt_regex").param_opt_regex("id", "[0-9]+"),
            param_opt_regex,
        );

        r.get(Path::new("param_glob").param_glob("id"), param_glob);
        r.get(
            Path::new("param_opt_glob").param_opt_glob("id"),
            param_opt_glob,
        );

        r.get(Path::new("param_typed").param::<i32>("id"), param_typed);

        r.scope(Scope::new(Path::new("scope_param").param("id")).to(|r| {
            r.get(Path::new("bar"), scope_param);
        }));

        r.scope(
            Scope::new(Path::new("scope_param_opt").param_opt("id")).to(|r| {
                r.get(Path::new("bar"), scope_param_opt);
            }),
        );
    }));

    //     get!(
    //         "param_glob_middle" / id: Vec<String> / "foo",
    //         param_glob_middle
    //     );
    //     get!(
    //         "param_optional_glob_middle" / id: Option<Vec<String>> / "foo",
    //         param_optional_glob_middle
    //     );

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
