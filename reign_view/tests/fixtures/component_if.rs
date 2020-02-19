write!(f, "{}", crate::views::Page {
    _slots: ::reign::view::Slots {
        templates: ::reign::view::maplit::hashmap! {},
        children: ::reign::view::slot_render(|f: &mut dyn std::fmt::Write| {
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
