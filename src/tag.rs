use nih_plug_vizia::vizia::prelude::*;

pub struct Tag {}

impl Tag {
    pub fn new(cx: &mut Context, content: impl Fn(&mut Context)) -> Handle<Self> {
        Self {}.build(cx, |cx| {
            HStack::new(cx, content);
        })
    }
}

impl View for Tag {
    fn element(&self) -> Option<&'static str> {
        Some("tag")
    }
}
