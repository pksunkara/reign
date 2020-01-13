write!(f, "{}", "<div")? ;
write!(f, " {}=\"{}\"", "x", format!("{}{}{}", "a", self.b, "c"))? ;
write!(f, " {}=\"{}\"", "y", format!("{}{}{}", "a", "b", "c"))? ;
write!(f, " {}=\"{}\"", "z", format!("{}{}{}", "a", self.b, "c"))? ;
write!(f, ">")? ;
write!(f, "{}", "</div>")? ;
