write!(
    f,
    "{}",
    crate::views::Page {
        _slots: ::reign::view::Slots {
            templates: ::maplit::hashmap! {
                "header" => Box::new(|f| {
                    write!(f, "{}", "Hello")? ;
                    Ok(())
                }),
                "footer" => Box::new(|f| {
                    write!(f, "{}", "Bye")? ;
                    Ok(())
                })
            },
            children: Box::new(|f| {
                write!(f, "{}", "\n  ")? ;
                write!(f, "{}", "\n  ")? ;
                write!(f, "{}", "<h1")? ;
                write!(f, ">")? ;
                write!(f, "{}", "Title")? ;
                write!(f, "{}", "</h1>")? ;
                write!(f, "{}", "\n  ")? ;
                write!(f, "{}", "\n")? ;
                Ok(())
            }),
            phantom: ::std::marker::PhantomData,
        },
        a: "b",
        c: self.d,
        e: "\"f\"",
        g: "h"
    }
)? ;
