use proc_macro2::TokenStream;
use quote::quote;

pub fn router(input: TokenStream) -> TokenStream {
    if cfg!(feature = "router-actix") {
        quote! {
            pub async fn router<A>(addr: A) -> std::io::Result<()>
            where
                A: std::net::ToSocketAddrs + Send + 'static
            {
                ::actix_web::HttpServer::new(|| {
                    let mut app = ::actix_web::App::new();

                    #input

                    app
                })
                .bind(addr)
                .unwrap()
                .run()
                .await
            }
        }
    } else if cfg!(feature = "router-gotham") {
        quote! {
            pub async fn router<A>(addr: A) -> Result<(), ()>
            where
                A: std::net::ToSocketAddrs + Send + 'static
            {
                use ::gotham::router::builder::*;

                ::gotham::init_server(
                    addr,
                    build_simple_router(|route| {
                        #input
                    })
                ).await
            }
        }
    } else if cfg!(feature = "router-tide") {
        quote! {
            pub async fn router<A>(addr: A) -> std::io::Result<()>
            where
                A: ::async_std::net::ToSocketAddrs + 'static
            {
                let mut app = ::tide::new();

                #input

                app.listen(addr).await
            }
        }
    } else {
        quote! {}
    }
}
