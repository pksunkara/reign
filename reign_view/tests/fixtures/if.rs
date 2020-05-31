write!(f, "{}", "<div")? ;
write!(f, ">")? ;
write!(f, "{}", "\n  ")? ;
if true {
    write!(f, "{}", "<div")? ;
    write!(f, ">")? ;
    write!(f, "{}", "First True")? ;
    write!(f, "{}", "</div>")? ;
}
write!(f, "{}", "\n  ")? ;
if self.a == "true" || self.a {
    write!(f, "{}", "<div")? ;
    write!(f, ">")? ;
    write!(f, "{}", "(Else) True")? ;
    write!(f, "{}", "</div>")? ;
} else if self.a == "false" || !self.a {
    write!(f, "{}", "<div")? ;
    write!(f, ">")? ;
    write!(f, "{}", "(Else) False")? ;
    write!(f, "{}", "</div>")? ;
} else {
    write!(f, "{}", "<div")? ;
    write!(f, ">")? ;
    write!(f, "{}", "Unknown")? ;
    write!(f, "{}", "</div>")? ;
}
write!(f, "{}", "\n  ")? ;
if self.a {
    write!(f, "{}", "<div")? ;
    write!(f, ">")? ;
    write!(f, "{}", "(ElseIf) True")? ;
    write!(f, "{}", "</div>")? ;
} else if !self.a {
    write!(f, "{}", "<div")? ;
    write!(f, ">")? ;
    write!(f, "{}", "(ElseIf) False")? ;
    write!(f, "{}", "</div>")? ;
}
write!(f, "{}", "\n  ")? ;
if true {
    write!(f, "{}", "<div")? ;
    write!(f, ">")? ;
    write!(f, "{}", "Last True")? ;
    write!(f, "{}", "</div>")? ;
}
write!(f, "{}", "\n")? ;
write!(f, "{}", "</div>")? ;
