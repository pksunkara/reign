use std::{
    any::{Any, TypeId},
    collections::HashMap as Map,
};

type AnyMap = Map<TypeId, Vec<Box<dyn Any + Send + Sync>>>;

#[derive(Debug)]
enum PathPart<'a> {
    Static(&'a str),
    Param(&'a str),
}

#[derive(Debug, Default)]
pub struct Path<'a> {
    parts: Vec<PathPart<'a>>,
    params: Option<Box<AnyMap>>,
}

impl<'a> Path<'a> {
    #[inline]
    pub fn empty() -> Self {
        Self::default()
    }

    pub fn new(name: &'a str) -> Self {
        let ret = Self::default();
        ret.path(name)
    }

    pub fn path(mut self, name: &'a str) -> Self {
        self.parts.push(PathPart::Static(name));
        self
    }

    pub fn param<T>(mut self, name: &'a str) -> Self {
        self.parts.push(PathPart::Param(name));
        self
    }

    pub fn param_opt<T>(mut self, name: &'a str) -> Self {
        self.parts.push(PathPart::Param(name));
        self
    }
}
