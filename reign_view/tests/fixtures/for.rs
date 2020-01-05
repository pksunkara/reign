write!(f, "{}", "<ul")? ;
write!(f, ">")? ;
write!(f, "{}", "\n  ")? ;
for i in 0..10 {
    write!(f, "{}", "<li")? ;
    write!(f, ">")? ;
    write!(f, "{}", "")? ;
    write!(f, "{}", "</li>")? ;
}
write!(f, "{}", "\n")? ;
write!(f, "{}", "</ul>")? ;
