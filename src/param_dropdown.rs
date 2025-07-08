use nih_plug::prelude::*;
use nih_plug_vizia::{
    vizia::{icons::ICON_CHEVRON_DOWN, prelude::*},
    widgets::param_base::ParamWidgetBase,
};

enum SelectorEvent {
    SetTo(f32),
}

#[derive(Lens)]
pub struct ParamDropdown {
    param_base: ParamWidgetBase,
}

impl ParamDropdown {
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
                Dropdown::new(
                    cx,
                    move |cx| {
                        HStack::new(cx, |cx| {
                            Label::new(
                                cx,
                                param_data.make_lens(|p| {
                                    p.normalized_value_to_string(
                                        p.modulated_normalized_value(),
                                        true,
                                    )
                                }),
                            )
                            .role(Role::PopupButton)
                            .cursor(CursorIcon::Hand)
                            .checked(PopupData::is_open)
                            .on_press(|cx| cx.emit(PopupEvent::Switch))
                            .class("value");
                            Label::new(cx, ICON_CHEVRON_DOWN)
                                .role(Role::PopupButton)
                                .cursor(CursorIcon::Hand)
                                .checked(PopupData::is_open)
                                .on_press(|cx| cx.emit(PopupEvent::Switch));
                        })
                    },
                    move |cx| {
                        let step_count = param_data.param().step_count().unwrap_or_default();

                        for value in (0..=step_count).map(|v| v as f32 / step_count as f32) {
                            let formatted =
                                param_data.param().normalized_value_to_string(value, false);

                            Label::new(cx, formatted.as_str())
                                .on_press(move |cx| {
                                    cx.emit(SelectorEvent::SetTo(value));
                                    cx.emit(PopupEvent::Close)
                                })
                                .toggle_class(
                                    "selected",
                                    param_data.make_lens(move |p| {
                                        p.modulated_normalized_value() == value
                                    }),
                                );
                        }
                    },
                );
            }),
        )
    }
}

impl View for ParamDropdown {
    fn element(&self) -> Option<&'static str> {
        Some("paramdropdown")
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
