use crate::{router::ty::subty_if_name, INTERNAL_ERR};

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

            let (fn_name, ty) = if let Some(ty) = subty_if_name(ty.clone(), "Vec") {
                (quote! { param_glob }, ty)
            } else if let Some(ty) = subty_if_name(ty.clone(), "Option") {
                if let Some(ty) = subty_if_name(ty.clone(), "Vec") {
                    (quote! { param_opt_glob }, ty)
                } else {
                    (quote! { param_opt }, ty)
                }
            } else {
                (quote! { param }, ty)
            };

            // TODO: Use respond on the param err instead of forwarding
            quote! {
                let #ident = #req_ident.#fn_name::<#ty>(#lit)?;
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
                    Err(e) => Ok(e.respond()?),
                }
            }.boxed()
        }
    }
}
