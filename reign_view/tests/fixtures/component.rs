write!(
    f,
    "{}",
    crate::views::Page {
        _slots: ::reign::view::Slots {
            templates: ::maplit::hashmap! {},
            children: Box::new(|f| {
                write!(f, "{}", "\n  ")? ;
                write!(f, "{}", "<h1")? ;
                write!(f, ">")? ;
                write!(f, "{}", "Title")? ;
                write!(f, "{}", "</h1>")? ;
                write!(f, "{}", "\n")? ;
                Ok(())
            }),
        },
        a: "b",
        c: self.d,
        e: "\"f\"",
        g: "h"
    }
)? ;
