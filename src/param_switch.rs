use nih_plug::prelude::*;
use nih_plug_vizia::{
    vizia::{icons::ICON_CHEVRON_DOWN, prelude::*},
    widgets::param_base::ParamWidgetBase,
};

enum SelectorEvent {
    SetTo(f32),
}

#[derive(Lens)]
pub struct ParamSwitch {
    param_base: ParamWidgetBase,
}

enum ParamSwitchEvent {
    Switch
}

impl ParamSwitch {
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
                Switch::new(
                    cx,
                    param_data.make_lens(|p| p.unmodulated_normalized_value() > 0.5)
                ).on_toggle(|cx| cx.emit(ParamSwitchEvent::Switch));
            }),
        )
    }
}

impl View for ParamSwitch {
    fn element(&self) -> Option<&'static str> {
        Some("paramswitch")
    }
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|param_slider_event, meta| match param_slider_event {
            ParamSwitchEvent::Switch => {
                let value = if self.param_base.unmodulated_normalized_value() > 0.5 { 0.0 } else { 1.0 };
                self.param_base.begin_set_parameter(cx);
                self.param_base.set_normalized_value(cx, value);
                self.param_base.end_set_parameter(cx);
                meta.consume();
            }
        });
    }
}
