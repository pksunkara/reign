write!(f, "{}", crate::views::Page {
    _slots: ::reign::view::Slots {
        templates: ::maplit::hashmap! {},
        children: Box::new (|f| {
            write!(f, "{}", "\n  " )? ;
            if self.a {
                write!(f, "{}", "<template" )? ;
                write!(f, ">" )? ;
                write!(f, "{}", "\n    " )? ;
                write!(f, "{}", "<h1" )? ;
                write!(f, ">" )? ;
                write!(f, "{}", "Yes" )? ;
                write!(f, "{}", "</h1>" )? ;
                write!(f, "{}", "\n    " )? ;
                write!(f, "{}", "<span" )? ;
                write!(f, ">" )? ;
                write!(f, "{}", "True" )? ;
                write!(f, "{}", "</span>" )? ;
                write!(f, "{}", "\n  " )? ;
                write!(f, "{}", "</template>" )? ;
            } else {
                write!(f, "{}", "<h1" )? ;
                write!(f, ">" )? ;
                write!(f, "{}", "Unknown" )? ;
                write!(f, "{}", "</h1>" )? ;
            }
            write!(f, "{}", "\n" )? ;
            Ok(())
        }),
    },
})? ;
