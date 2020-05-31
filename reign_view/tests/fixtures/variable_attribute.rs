write!(f, "{}", "<div")? ;
write!(f, " {}=\"{}\"", "title", self.title)? ;
write!(f, " {}=\"{}\"", "a", format!("{}_b", 1))? ;
write!(f, " {}=\"{}\"", "x", self.y)? ;
write!(f, ">")? ;
write!(f, "{}", "</div>")? ;
