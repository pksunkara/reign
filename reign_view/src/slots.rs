use std::collections::HashMap;
use std::fmt::{Result, Write};

pub type SlotRender = Box<dyn Fn(&mut dyn Write) -> Result>;

pub struct Slots {
    pub templates: HashMap<String, SlotRender>,
    pub children: SlotRender,
}

impl Slots {
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

impl Default for Slots {
    fn default() -> Self {
        Slots {
            templates: HashMap::new(),
            children: Box::new(|_| Ok(())),
        }
    }
}
