use std::collections::HashMap;
use std::fmt::{Result, Write};

pub type SlotRender = Box<dyn FnMut(&mut dyn Write) -> Result>;

pub struct Slots {
    templates: HashMap<String, SlotRender>,
    children: SlotRender,
}

impl Slots {
    pub fn render(&mut self, f: &mut dyn Write, name: &str) -> Result {
        if let Some(func) = self.templates.get_mut(name) {
            func(f)
        } else if name == "default" {
            (self.children)(f)
        } else {
            Ok(())
        }
    }
}
