write!(
    f,
    "{}",
    crate::views::Page {
        _slots: ::reign::view::Slots {
            templates: ::reign::view::maplit::hashmap! {
                "header" => ::reign::view::slot_render(|f: &mut dyn std::fmt::Write| {
                    write!(f, "{}", "Hello")? ;
                    Ok(())
                }),
                "footer" => ::reign::view::slot_render(|f: &mut dyn std::fmt::Write| {
                    write!(f, "{}", "Bye")? ;
                    Ok(())
                })
            },
            children: ::reign::view::slot_render(|f: &mut dyn std::fmt::Write| {
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
