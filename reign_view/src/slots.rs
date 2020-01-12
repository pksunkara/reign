use std::collections::HashMap;
use std::fmt::{Result, Write};
use std::marker::PhantomData;

pub type SlotRender = Box<dyn Fn(&mut dyn Write) -> Result>;

pub struct Slots<'a> {
    pub templates: HashMap<String, SlotRender>,
    pub children: SlotRender,
    pub phantom: PhantomData<&'a str>,
}

impl<'a> Slots<'a> {
    pub fn render(&self, f: &mut dyn Write, name: &str) -> Result {
        if let Some(func) = self.templates.get(name) {
            func(f)
        } else if name == "default" {
            (self.children)(f)
        } else {
            Ok(())
        }
    }
}

impl<'a> Default for Slots<'a> {
    fn default() -> Self {
        Slots {
            templates: HashMap::new(),
            children: Box::new(|_| Ok(())),
            phantom: PhantomData,
        }
    }
}
