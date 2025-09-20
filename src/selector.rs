use nih_plug::params::enums::Enum;
use nih_plug_vizia::vizia::prelude::*;

enum SelectorEvent {
    Update(usize),
}

#[derive(Lens)]
pub struct Selector {
    on_toggle_action: Option<Box<dyn Fn(&mut EventContext, usize)>>,
}

impl Selector {
    pub fn new<E: Enum + Clone>(cx: &mut Context, data: impl Lens<Target = E>) -> Handle<Self> {
        Self {
            on_toggle_action: None,
        }
        .build(cx, |cx| {
            HStack::new(cx, |cx| {
                for (i, variant) in E::variants().iter().enumerate() {
                    Button::new(cx, |_| {}, |cx| Label::new(cx, *variant))
                        .on_press(move |cx| cx.emit(SelectorEvent::Update(i)))
                        .toggle_class("on", data.map(move |d| d.clone().to_index() == i));
                }
            });
        })
    }
}

pub trait SelectorModifiers {
    fn on_toggle<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, usize);
}

impl SelectorModifiers for Handle<'_, Selector> {
    fn on_toggle<F>(self, callback: F) -> Self
    where
        F: 'static + Fn(&mut EventContext, usize),
    {
        self.modify(|selector| selector.on_toggle_action = Some(Box::new(callback)))
    }
}

impl View for Selector {
    fn element(&self) -> Option<&'static str> {
        Some("selector")
    }
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|selector_event, _| match selector_event {
            SelectorEvent::Update(i) => {
                if let Some(ref f) = self.on_toggle_action {
                    f(cx, *i);
                }
            }
        });
    }
}
