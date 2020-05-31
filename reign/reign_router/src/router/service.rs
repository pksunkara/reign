use crate::router::{
    hyper::{Body, Request as HyperRequest, Response as HyperResponse, StatusCode},
    Chain, Constraint, Error, Handler, MiddlewareItem, Request, Router, INTERNAL_ERR,
};
use log::debug;
use regex::{Regex, RegexSet};
use std::{collections::HashMap as Map, net::SocketAddr, sync::Arc};

pub(crate) struct RouteRef {
    pub(crate) handler: Option<Arc<Handler>>,
    pub(crate) middlewares: Vec<Arc<MiddlewareItem>>,
    pub(crate) constraints: Vec<Option<Arc<Constraint>>>,
}

#[derive(Clone)]
pub struct Service<'a> {
    router: Arc<Router<'a>>,
    regexes: Arc<Vec<Regex>>,
    regex_set: Arc<RegexSet>,
    refs: Arc<Vec<RouteRef>>,
}

impl<'a> Service<'a> {
    pub(crate) fn new(router: Router<'a>) -> Self {
        let refs = router.refs();

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

    pub async fn call(
        self,
        req: HyperRequest<Body>,
        ip: SocketAddr,
    ) -> Result<HyperResponse<Body>, Error> {
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

        // TODO:(router:404) Check for 405 and support custom error handler through post middleware
        Ok(HyperResponse::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty())?)
    }

    async fn run(
        handler: &Arc<Handler>,
        mut request: Request,
        route: &RouteRef,
    ) -> Result<HyperResponse<Body>, Error> {
        let chain = Chain {
            handler,
            middlewares: &route.middlewares,
        };

        chain.run(&mut request).await
    }
}

pub fn service<'a, R>(router_fn: R) -> Service<'a>
where
    R: Fn(&mut Router),
{
    let mut router = Router::default();
    router_fn(&mut router);

    Service::new(router)
}
