write!(f, "{}", "<ul")? ;
write!(f, ">")? ;
write!(f, "{}", "\n  ")? ;
for (i, j, _) in self.users {
    write!(f, "{}", "<li")? ;
    write!(f, ">")? ;
    write!(f, "{}{}{}", i, j, self.k)? ;
    write!(f, "{}", "</li>")? ;
}
write!(f, "{}", "\n  ")? ;
for User { i, b: j, ref k, d: &l, .. } in self.users {
    write!(f, "{}", "<li")? ;
    write!(f, ">")? ;
    write!(f, "{}{}{}{}", i, j, k, l)? ;
    write!(f, "{}", "</li>")? ;
}
write!(f, "{}", "\n  ")? ;
for i in self.users {
    write!(f, "{}", "<li")? ;
    write!(f, ">")? ;
    write!(f, "{}", "\n    ")? ;
    write!(f, "{}", "<h1")? ;
    write!(f, ">")? ;
    write!(f, "{}", i)? ;
    write!(f, "{}", "</h1>")? ;
    write!(f, "{}", "\n    ")? ;
    write!(f, "{}", "<ul")? ;
    write!(f, ">")? ;
    write!(f, "{}", "\n      ")? ;
    for j in i {
        write!(f, "{}", "<li")? ;
        write!(f, ">")? ;
        write!(f, "{}{}", i, j)? ;
        write!(f, "{}", "</li>")? ;
    }
    write!(f, "{}", "\n    ")? ;
    write!(f, "{}", "</ul>")? ;
    write!(f, "{}", "\n  ")? ;
    write!(f, "{}", "</li>")? ;
}
write!(f, "{}", "\n")? ;
write!(f, "{}", "</ul>")? ;
