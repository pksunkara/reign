use std::collections::HashMap;
use std::fmt::{Result, Write};
use std::marker::PhantomData;

type SlotRender<'a> = Box<dyn Fn(&mut dyn Write) -> Result + 'a>;

pub fn slot_render<'a, F>(f: F) -> SlotRender<'a>
where
    F: Fn(&mut dyn Write) -> Result + 'a,
{
    Box::new(f) as SlotRender
}

pub struct Slots<'a> {
    pub templates: HashMap<&'a str, SlotRender<'a>>,
    pub children: SlotRender<'a>,
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
            children: slot_render(|_| Ok(())),
            phantom: PhantomData,
        }
    }
}
