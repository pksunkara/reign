use syn::{GenericArgument, Path, PathArguments, PathSegment, Type, TypeGroup, TypePath};

pub fn subty_if_name(ty: Type, name: &str) -> Option<Type> {
    subty_if(ty, |seg| seg.ident == name)
}

fn subty_if<F>(ty: Type, f: F) -> Option<Type>
where
    F: FnOnce(&PathSegment) -> bool,
{
    only_last_segment(ty)
        .filter(|segment| f(segment))
        .and_then(|segment| {
            if let PathArguments::AngleBracketed(args) = segment.arguments {
                only_one(args.args.into_iter()).and_then(|genneric| {
                    if let GenericArgument::Type(ty) = genneric {
                        Some(ty)
                    } else {
                        None
                    }
                })
            } else {
                None
            }
        })
}

pub fn only_last_segment(mut ty: Type) -> Option<PathSegment> {
    while let Type::Group(TypeGroup { elem, .. }) = ty {
        ty = *elem;
    }
    match ty {
        Type::Path(TypePath {
            qself: None,
            path:
                Path {
                    leading_colon: None,
                    segments,
                },
        }) => only_one(segments.into_iter()),
        _ => None,
    }
}

pub fn only_one<I, T>(mut iter: I) -> Option<T>
where
    I: Iterator<Item = T>,
{
    iter.next().filter(|_| iter.next().is_none())
}
