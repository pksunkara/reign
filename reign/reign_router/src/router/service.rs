use crate::router::{
    hyper::{Body, Request as HyperRequest, Response as HyperResponse, StatusCode},
    Constraint, Error, Handler, Request, Router, INTERNAL_ERR,
};
use log::debug;
use regex::{Regex, RegexSet};
use std::{collections::HashMap as Map, net::SocketAddr, pin::Pin, sync::Arc};

pub(crate) struct RouteRef {
    pub(crate) handler: Option<Arc<Handler>>,
    pub(crate) constraints: Vec<Option<Arc<Constraint>>>,
}

#[derive(Clone)]
pub(crate) struct Service<'a> {
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

    pub(crate) async fn call_with_addr(
        self,
        req: HyperRequest<Body>,
        ip: SocketAddr,
    ) -> Result<HyperResponse<Body>, Error> {
        let to_match = format!("{}{}", req.method().as_str(), req.uri().path());
        let to_match = to_match.trim_end_matches('/');
        let matches = self.regex_set.matches(to_match);

        if !matches.matched_any() {
            // TODO:(router:404) Check for 405 and support custom error handler through post middleware
            return Ok(HyperResponse::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Body::empty())?);
        }

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
                    return Pin::from(handler(request)).await;
                }
            }
        }

        Ok(HyperResponse::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .body(Body::empty())?)
    }
}
