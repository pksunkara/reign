write!(f, "{}", "<div")? ;
write!(f, ">")? ;
write!(f, "{}", "\n  ")? ;
if true {
    write!(f, "{}", "<div")? ;
    write!(f, ">")? ;
    write!(f, "{}", "True")? ;
    write!(f, "{}", "</div>")? ;
} else if true {
    write!(f, "{}", "<div")? ;
    write!(f, ">")? ;
    write!(f, "{}", "False")? ;
    write!(f, "{}", "</div>")? ;
} else {
    write!(f, "{}", "<div")? ;
    write!(f, ">")? ;
    write!(f, "{}", "Unknown")? ;
    write!(f, "{}", "</div>")? ;
}
write!(f, "{}", "\n")? ;
write!(f, "{}", "</div>")? ;
