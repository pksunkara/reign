use crate::{Chain, Constraint, Handler, MiddlewareItem, Request, Response, Router, INTERNAL_ERR};
use hyper::{
    http::Error as HttpError, Body, Request as HyperRequest, Response as HyperResponse, StatusCode,
};
use log::{debug, error, trace};
use regex::{Regex, RegexSet};

use std::{collections::HashMap as Map, net::SocketAddr, sync::Arc};

pub(crate) struct RouteRef {
    pub(crate) handler: Option<Arc<Handler>>,
    pub(crate) middlewares: Vec<Arc<MiddlewareItem>>,
    pub(crate) constraints: Vec<Option<Arc<Constraint>>>,
}

/// Thread safe structure that optimizes the given router for responding to requests
#[derive(Clone)]
pub struct Service {
    router: Arc<Router>,
    regexes: Arc<Vec<Regex>>,
    regex_set: Arc<RegexSet>,
    refs: Arc<Vec<RouteRef>>,
}

impl Service {
    pub(crate) fn new(router: Router) -> Self {
        let refs = router.refs(Map::new());

        let regexes = router
            .regex()
            .iter()
            .map(|x| format!("{}{}", x.0, x.1))
            .collect::<Vec<_>>();

        debug!("Route regexes: {:?}", regexes);

        Self {
            router: Arc::new(router),
            regexes: Arc::new(
                regexes
                    .iter()
                    .map(|x| Regex::new(x).expect(INTERNAL_ERR))
                    .collect(),
            ),
            regex_set: Arc::new(RegexSet::new(regexes).expect(INTERNAL_ERR)),
            refs: Arc::new(refs),
        }
    }

    /// Respond to a given hyper Request and IP address
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use reign::router::{service, Router};
    ///
    /// fn router(r: &mut Router) {}
    ///
    /// #[tokio::test]
    /// async fn test() {
    ///     let service = service(router);
    ///
    ///     let response = service
    ///         .call(
    ///             Req::get("https://reign.rs/get")
    ///                 .body(Body::empty())
    ///                 .unwrap(),
    ///             "10.10.10.10:80".parse().unwrap(),
    ///         )
    ///         .await
    ///         .unwrap();
    /// }
    /// ```
    pub async fn call(
        self,
        req: HyperRequest<Body>,
        ip: SocketAddr,
    ) -> Result<HyperResponse<Body>, HttpError> {
        trace!("Incoming request to router");

        let to_match = format!("{}{}", req.method().as_str(), req.uri().path());
        let to_match = to_match.trim_end_matches('/');
        let matches = self.regex_set.matches(to_match);

        let mut request = Request::new(ip, req);

        for m in matches {
            let regex = self.regexes.get(m).expect(INTERNAL_ERR);

            debug!("Checking regex: {:?}", regex);

            let mut params = Map::new();
            let captures = regex.captures(to_match).expect(INTERNAL_ERR);

            for name in regex.capture_names() {
                if let Some(name) = name {
                    if let Some(value) = captures.name(name) {
                        params.insert(name.to_string(), value.as_str().to_string());
                    }
                }
            }

            debug!("Params extracted: {:?}", params);

            request.params = params;

            if let Some(route) = self.refs.get(m) {
                let mut matched = true;

                for constraint in &route.constraints {
                    if let Some(constraint) = constraint {
                        matched = constraint(&request);

                        if !matched {
                            break;
                        }
                    }
                }

                if !matched {
                    continue;
                }

                if let Some(handler) = &route.handler {
                    return Self::run(handler, request, route).await;
                }
            }
        }

        // TODO: router: Check for 405 and support custom error handler through post middleware
        // Can make this a special error or make a special middleware pipeline for errors
        Ok(HyperResponse::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty())?)
    }

    async fn run(
        handler: &Arc<Handler>,
        mut request: Request,
        route: &RouteRef,
    ) -> Result<HyperResponse<Body>, HttpError> {
        let chain = Chain {
            handler,
            middlewares: &route.middlewares,
        };

        match chain.run(&mut request).await {
            Ok(r) => Ok(r),
            Err(err) => {
                error!("{}", err);
                err.respond()
            }
        }
    }
}

/// Converts the router into a service that responds to a given hyper Request
///
/// Useful in tests without needing to spin up the server
///
/// # Examples
///
/// ```no_run
/// use reign::router::{service, Router};
///
/// fn router(r: &mut Router) {}
///
/// #[tokio::test]
/// async fn test() {
///     let service = service(router);
///
///     let response = service
///         .call(
///             Req::get("https://reign.rs/get")
///                 .body(Body::empty())
///                 .unwrap(),
///             "10.10.10.10:80".parse().unwrap(),
///         )
///         .await
///         .unwrap();
/// }
/// ```
pub fn service<R>(f: R) -> Service
where
    R: FnOnce(&mut Router),
{
    let mut router = Router::default();
    f(&mut router);

    Service::new(router)
}
