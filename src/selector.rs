use nih_plug::prelude::*;
use nih_plug_vizia::{vizia::prelude::*, widgets::param_base::ParamWidgetBase};

enum SelectorEvent {
    SetTo(f32),
}

#[derive(Lens)]
pub struct Selector {
    param_base: ParamWidgetBase,
}

impl Selector {
    pub fn new<L, Params, P, FMap>(
        cx: &mut Context,
        params: L,
        params_to_param: FMap,
    ) -> Handle<Self>
    where
        L: Lens<Target = Params> + Clone,
        Params: 'static,
        P: Param + 'static,
        FMap: Fn(&Params) -> &P + Copy + 'static,
    {
        Self {
            param_base: ParamWidgetBase::new(cx, params, params_to_param),
        }
        .build(
            cx,
            ParamWidgetBase::build_view(params, params_to_param, move |cx, param_data| {
                HStack::new(cx, |cx| {
                    let step_count = param_data.param().step_count().unwrap_or_default();
                    for value in (0..=step_count).map(|v| v as f32 / step_count as f32) {
                        let formatted = param_data.param().normalized_value_to_string(value, false);
                        Button::new(cx, |_| {}, |cx| Label::new(cx, formatted.as_str()))
                            .on_press(move |cx| cx.emit(SelectorEvent::SetTo(value)))
                            .toggle_class(
                                "on",
                                param_data
                                    .make_lens(move |p| p.modulated_normalized_value() == value),
                            );
                    }
                });
            }),
        )
    }
}

impl View for Selector {
    fn element(&self) -> Option<&'static str> {
        Some("selector")
    }
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|param_slider_event, meta| match param_slider_event {
            SelectorEvent::SetTo(x) => {
                self.param_base.begin_set_parameter(cx);
                self.param_base.set_normalized_value(cx, *x);
                self.param_base.end_set_parameter(cx);
                meta.consume();
            }
        });
    }
}
