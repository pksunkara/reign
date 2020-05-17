use proc_macro2::TokenStream;
use quote::quote;
use syn::ItemFn;

pub fn router(input: ItemFn) -> TokenStream {
    let ItemFn {
        attrs, sig, block, ..
    } = input;

    let name = sig.ident;
    let stmts = block.stmts;

    if cfg!(feature = "router-actix") {
        quote! {
            #(#attrs)*
            pub async fn #name<A>(addr: A) -> std::io::Result<()>
            where
                A: std::net::ToSocketAddrs + Send + 'static
            {
                ::actix_web::HttpServer::new(|| {
                    let mut app = ::actix_web::App::new();

                    #(#stmts)*

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
            #(#attrs)*
            pub async fn #name<A>(addr: A) -> Result<(), ()>
            where
                A: std::net::ToSocketAddrs + Send + 'static
            {
                use ::gotham::router::builder::*;

                ::gotham::init_server(
                    addr,
                    build_simple_router(|route| {
                        #(#stmts)*
                    })
                ).await
            }
        }
    } else if cfg!(feature = "router-tide") {
        quote! {
            #(#attrs)*
            pub async fn #name<A>(addr: A) -> std::io::Result<()>
            where
                A: std::net::ToSocketAddrs + Send + 'static
            {
                let mut app = ::tide::new();

                #(#stmts)*

                app.listen(addr).await
            }
        }
    } else {
        quote! {
            #(#attrs)*
            pub async fn #name<A>(addr: A) -> std::io::Result<()>
            where
                A: ::async_std::net::ToSocketAddrs + 'static
            {
                #(#stmts)*
            }
        }
    }
}
