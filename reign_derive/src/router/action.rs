use crate::{
    router::ty::{only_last_segment, subty_if_name},
    INTERNAL_ERR,
};
use proc_macro2::TokenStream;
use proc_macro_error::abort;
use quote::quote;
use syn::{spanned::Spanned, FnArg, Ident, ItemFn, LitStr, Pat, Signature, Type};

fn arg_ident(arg: &FnArg) -> Ident {
    if let FnArg::Typed(x) = arg {
        if let Pat::Ident(x) = &*x.pat {
            return x.ident.clone();
        }
    }

    abort!(arg.span(), "expected a typed function arg with clear ident");
}

fn arg_ty(arg: &FnArg) -> Type {
    if let FnArg::Typed(x) = arg {
        return (*x.ty).clone();
    }

    abort!(arg.span(), "expected a typed function arg with clear ident");
}

pub fn action(input: ItemFn) -> TokenStream {
    let ItemFn {
        attrs,
        sig,
        block,
        vis,
    } = input;
    let Signature {
        ident,
        inputs,
        constness,
        unsafety,
        fn_token,
        output,
        ..
    } = sig;

    let args = inputs.iter().map(|x| arg_ident(x)).collect::<Vec<_>>();

    let req = if let Some(arg) = inputs.first() {
        arg
    } else {
        abort!(
            inputs.span(),
            "expected atleast one argument denoting Request"
        );
    };

    let req_ident = args.first().expect(INTERNAL_ERR);
    let idents = args.iter().skip(1).collect::<Vec<_>>();
    let assignments = inputs
        .iter()
        .skip(1)
        .map(|x| {
            let ident = arg_ident(x);
            let lit = LitStr::new(&ident.to_string(), ident.span());
            let ty = arg_ty(x);

            let (fn_name, ty, glob, opt) = if let Some(ty) = subty_if_name(ty.clone(), "Vec") {
                (quote! { param_glob }, ty, true, false)
            } else if let Some(ty) = subty_if_name(ty.clone(), "Option") {
                if let Some(ty) = subty_if_name(ty.clone(), "Vec") {
                    (quote! { param_opt_glob }, ty, true, true)
                } else {
                    (quote! { param_opt }, ty, false, true)
                }
            } else {
                (quote! { param }, ty, false, false)
            };

            let run_fn = quote! {
                let #ident = #req_ident.#fn_name(#lit)?;
            };

            if only_last_segment(ty.clone())
                .map(|x| x.ident == "String")
                .unwrap_or(false)
            {
                return run_fn;
            }

            let context = quote! {
                .context("Unable to convert param to type")?
            };

            let from_str = if glob && opt {
                quote! {
                    let #ident = if let Some(#ident) = #ident {
                        Some(#ident.iter().map(<#ty as std::str::FromStr>::from_str).collect::<Result<Vec<_>, _>>()#context)
                    } else {
                         None
                    };
                }
            } else if opt {
                quote! {
                    let #ident = if let Some(#ident) = #ident {
                        Some(<#ty as std::str::FromStr>::from_str(&#ident)#context)
                    } else {
                        None
                    };
                }
            } else if glob {
                quote! {
                    let #ident = #ident.iter().map(<#ty as std::str::FromStr>::from_str).collect::<Result<Vec<_>, _>>()#context;
                }
            } else {
                quote! {
                    let #ident = <#ty as std::str::FromStr>::from_str(&#ident)#context;
                }
            };

            quote! {
                use ::anyhow::Context;
                #run_fn
                #from_str
            }
        })
        .collect::<Vec<_>>();

    quote! {
        #(#attrs)*
        #vis #constness #unsafety #fn_token #ident(
            #req
        ) -> ::reign::router::HandleFuture {
            use ::reign::router::futures::FutureExt;

            async move {
                #[inline]
                async fn _call(
                    #inputs
                ) #output #block

                #(#assignments)*

                let _called = _call(#req_ident, #(#idents),*).await;

                match _called {
                    Ok(r) => Ok(r.respond()?),
                    Err(e) => {
                        ::reign::log::error!("{}", e);
                        Ok(e.respond()?)
                    },
                }
            }.boxed()
        }
    }
}
