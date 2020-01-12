write!(f, "{}", crate::views::Page {
    _slots: ::reign::view::Slots {
        templates: ::maplit::hashmap! {},
        children: Box::new (|f| {
            write!(f, "{}", "\n  " )? ;
            if self.a {
                write!(f, "{}", "<h1" )? ;
                write!(f, ">" )? ;
                write!(f, "{}", "True" )? ;
                write!(f, "{}", "</h1>" )? ;
            } else {
                write!(f, "{}", "<h1" )? ;
                write!(f, ">" )? ;
                write!(f, "{}", "False" )? ;
                write!(f, "{}", "</h1>" )? ;
            }
            write!(f, "{}", "\n" )? ;
            Ok(())
        }),
        phantom: ::std::marker::PhantomData,
    },
})? ;
