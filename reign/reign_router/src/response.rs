#[cfg(feature = "router-actix")]
use actix_web::{Error, HttpRequest, HttpResponse, Responder};
#[cfg(feature = "router-actix")]
use futures::future::Ready;
#[cfg(feature = "router-gotham")]
use gotham::{
    handler::IntoResponse,
    hyper::{Body, Response as HyperResponse},
    state::State,
};
#[cfg(feature = "router-tide")]
use tide::Response as TideResponse;

pub trait Response: Sized {
    #[cfg(feature = "router-actix")]
    fn actix_response(self, _req: &HttpRequest) -> Ready<Result<HttpResponse, Error>> {
        unimplemented!()
    }

    #[cfg(feature = "router-gotham")]
    fn gotham_response(self, _state: &State) -> HyperResponse<Body> {
        unimplemented!()
    }

    #[cfg(feature = "router-tide")]
    fn tide_response(self) -> TideResponse {
        unimplemented!()
    }
}

macro_rules! derive_response {
    ($type:ty) => {
        impl Response for $type {
            #[cfg(feature = "router-actix")]
            #[inline]
            fn actix_response(self, req: &HttpRequest) -> Ready<Result<HttpResponse, Error>> {
                self.respond_to(req)
            }

            #[cfg(feature = "router-gotham")]
            #[inline]
            fn gotham_response(self, state: &State) -> HyperResponse<Body> {
                self.into_response(state)
            }

            #[cfg(feature = "router-tide")]
            #[inline]
            fn tide_response(self) -> TideResponse {
                self.into()
            }
        }
    };
}

derive_response!(&'static str);
derive_response!(String);
// TODO:(router:response) Bytes, [u8]

#[cfg(feature = "router-actix")]
impl Response for HttpResponse {
    #[inline]
    fn actix_response(self, req: &HttpRequest) -> Ready<Result<HttpResponse, Error>> {
        self.respond_to(req)
    }
}

#[cfg(feature = "router-gotham")]
impl Response for HyperResponse<Body> {
    #[inline]
    fn gotham_response(self, state: &State) -> HyperResponse<Body> {
        self.into_response(state)
    }
}

#[cfg(feature = "router-tide")]
impl Response for TideResponse {
    #[inline]
    fn tide_response(self) -> TideResponse {
        self
    }
}
