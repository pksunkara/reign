enum PathPart<'a> {
    Static(&'a str),
}

pub struct Path<'a> {
    parts: Vec<PathPart<'a>>,
}

impl<'a> Path<'a> {}
