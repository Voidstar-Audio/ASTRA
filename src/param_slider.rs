use nih_plug::prelude::*;
use nih_plug_vizia::{
    vizia::prelude::*,
    widgets::{param_base::ParamWidgetBase, util::*},
};

#[derive(Lens)]
pub struct ParamSlider {
    param_base: ParamWidgetBase,
    dragging: bool,
    granular_drag_status: Option<GranularDragStatus>,
    pub text_input_active: bool,
    scrolled_lines: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Data)]
pub enum SliderStyle {
    Centered,
    FromLeft,
    FromMidPoint,
    CurrentStep { even: bool },
    CurrentStepLabeled { even: bool },
}

pub struct SliderTick {
    pub pos: f32,
    pub label: Option<String>,
    pub short: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct GranularDragStatus {
    pub starting_x_coordinate: f32,
    pub starting_value: f32,
}
const GRANULAR_DRAG_MULTIPLIER: f32 = 0.1;

impl ParamSlider {
    pub fn new<L, Params, P, FMap>(
        cx: &mut Context,
        params: L,
        params_to_param: FMap,
        ticks: impl IntoIterator<Item = SliderTick>,
    ) -> Handle<Self>
    where
        L: Lens<Target = Params> + Clone,
        Params: 'static,
        P: Param + 'static,
        FMap: Fn(&Params) -> &P + Copy + 'static,
    {
        let style = SliderStyle::Centered;

        Self {
            param_base: ParamWidgetBase::new(cx, params, params_to_param),
            dragging: false,
            granular_drag_status: None,
            text_input_active: false,
            scrolled_lines: 0.0,
        }
        .build(
            cx,
            ParamWidgetBase::build_view(params, params_to_param, move |cx, param_data| {
                let unmodulated_normalized_value_lens =
                    param_data.make_lens(|param| param.unmodulated_normalized_value());
                let display_value_lens = param_data.make_lens(|param| {
                    param.normalized_value_to_string(param.unmodulated_normalized_value(), true)
                });

                let fill_start_delta_lens =
                    unmodulated_normalized_value_lens.map(move |current_value| {
                        Self::compute_fill_start_delta(style, param_data.param(), *current_value)
                    });

                // let modulation_start_delta_lens = param_data.make_lens(move |param| {
                //     Self::compute_modulation_fill_start_delta(style, param)
                // });

                HStack::new(cx, |cx| {
                    Label::new(cx, param_data.param().name().to_uppercase().as_str()).class("name");
                    Binding::new(
                        cx,
                        ParamSlider::text_input_active,
                        move |cx, text_input_active| {
                            if text_input_active.get(cx) {
                                Textbox::new(cx, display_value_lens)
                                    .on_submit(|cx, string, success| {
                                        if success {
                                            cx.emit(SliderEvent::TextInput(string))
                                        } else {
                                            cx.emit(SliderEvent::CancelTextInput);
                                        }
                                        // TODO: Make this dependant on whether focus was visible for textarea
                                        cx.focus();
                                    })
                                    .on_focus_out(|cx| {
                                        cx.emit(SliderEvent::CancelTextInput);
                                    })
                                    .on_cancel(|cx| {
                                        cx.emit(SliderEvent::CancelTextInput);
                                        // TODO: Make this dependant on whether focus was visible for textarea
                                        cx.focus();
                                    })
                                    .on_build(|cx| {
                                        cx.emit(TextEvent::StartEdit);
                                        cx.emit(TextEvent::SelectAll);
                                    });
                            } else {
                                Label::new(cx, display_value_lens).class("value");
                            }
                        },
                    );
                })
                .class("title");

                ZStack::new(cx, |cx| {
                    ZStack::new(cx, |cx| {
                        Element::new(cx)
                            .height(Stretch(1.0))
                            .left(
                                fill_start_delta_lens
                                    .map(|(start_t, _)| Percentage(start_t * 100.0)),
                            )
                            .width(
                                fill_start_delta_lens.map(|(_, delta)| Percentage(delta * 100.0)),
                            )
                            .translate(Translate::new(Pixels(0.5), Pixels(0.0)))
                            .class("slider")
                            .hoverable(false);
                        Element::new(cx)
                            .height(Stretch(1.0))
                            .width(Pixels(1.0))
                            .translate(Translate::new(Pixels(-0.5), Pixels(0.0)))
                            .left(unmodulated_normalized_value_lens.map(|x| Percentage(x * 100.0)))
                            .class("head");
                    })
                    .overflow(Overflow::Hidden);
                })
                .class("track");

                let mut ticks = ticks.into_iter().peekable();

                if ticks.peek().is_some() {
                    ZStack::new(cx, |cx| {
                        for tick in ticks {
                            fn tickmark<'a>(
                                cx: &'a mut Context,
                                tick_short: &bool,
                            ) -> Handle<'a, Element> {
                                Element::new(cx)
                                    .class("tick")
                                    .toggle_class("short", *tick_short)
                            }

                            if let Some(label) = tick.label.as_ref() {
                                VStack::new(cx, move |cx| {
                                    tickmark(cx, &tick.short);
                                    Label::new(cx, label)
                                        .class("tick-label")
                                        .width(Pixels(1.0))
                                        .text_align(TextAlign::Center);
                                })
                                .width(Pixels(1.0))
                                .height(Auto)
                                .left(Units::Percentage(tick.pos * 100.0));
                            } else {
                                tickmark(cx, &tick.short).left(Units::Percentage(tick.pos * 100.0));
                            }
                        }
                    })
                    .class("ticks");
                }
            }),
        )
        .navigable(true)
    }

    /// Calculate the start position and width of the slider's fill region based on the selected
    /// style, the parameter's current value, and the parameter's step sizes. The resulting tuple
    /// `(start_t, delta)` corresponds to the start and the signed width of the bar. `start_t` is in
    /// `[0, 1]`, and `delta` is in `[-1, 1]`.
    fn compute_fill_start_delta<P: Param>(
        style: SliderStyle,
        param: &P,
        current_value: f32,
    ) -> (f32, f32) {
        let default_value = param.default_normalized_value();
        let step_count = param.step_count();
        let draw_fill_from_default = matches!(style, SliderStyle::Centered)
            && step_count.is_none()
            && (0.45..=0.55).contains(&default_value);

        match style {
            SliderStyle::Centered if draw_fill_from_default => {
                let delta = (default_value - current_value).abs();

                // Don't draw the filled portion at all if it could have been a
                // rounding error since those slivers just look weird
                (
                    default_value.min(current_value),
                    if delta >= 1e-3 { delta } else { 0.0 },
                )
            }
            SliderStyle::FromMidPoint => {
                let delta = (0.5 - current_value).abs();

                // Don't draw the filled portion at all if it could have been a
                // rounding error since those slivers just look weird
                (
                    0.5_f32.min(current_value),
                    if delta >= 1e-3 { delta } else { 0.0 },
                )
            }
            SliderStyle::Centered | SliderStyle::FromLeft => (0.0, current_value),
            SliderStyle::CurrentStep { even: true }
            | SliderStyle::CurrentStepLabeled { even: true }
                if step_count.is_some() =>
            {
                // Assume the normalized value is distributed evenly
                // across the range.
                let step_count = step_count.unwrap() as f32;
                let discrete_values = step_count + 1.0;
                let previous_step = (current_value * step_count) / discrete_values;

                (previous_step, discrete_values.recip())
            }
            SliderStyle::CurrentStep { .. } | SliderStyle::CurrentStepLabeled { .. } => {
                let previous_step = param.previous_normalized_step(current_value, false);
                let next_step = param.next_normalized_step(current_value, false);

                (
                    (previous_step + current_value) / 2.0,
                    ((next_step - current_value) + (current_value - previous_step)) / 2.0,
                )
            }
        }
    }

    /// The same as `compute_fill_start_delta`, but just showing the modulation offset.
    fn compute_modulation_fill_start_delta<P: Param>(style: SliderStyle, param: &P) -> (f32, f32) {
        match style {
            // Don't show modulation for stepped parameters since it wouldn't
            // make a lot of sense visually
            SliderStyle::CurrentStep { .. } | SliderStyle::CurrentStepLabeled { .. } => (0.0, 0.0),
            SliderStyle::Centered | SliderStyle::FromMidPoint | SliderStyle::FromLeft => {
                let modulation_start = param.unmodulated_normalized_value();

                (
                    modulation_start,
                    param.modulated_normalized_value() - modulation_start,
                )
            }
        }
    }

    /// `self.param_base.set_normalized_value()`, but resulting from a mouse drag. When using the
    /// 'even' stepped slider styles from [`ParamSliderStyle`] this will remap the normalized range
    /// to match up with the fill value display. This still needs to be wrapped in a parameter
    /// automation gesture.
    fn set_normalized_value_drag(&self, cx: &mut EventContext, normalized_value: f32) {
        self.param_base.set_normalized_value(cx, normalized_value);
    }
}
enum SliderEvent {
    CancelTextInput,
    TextInput(String),
}

impl View for ParamSlider {
    fn element(&self) -> Option<&'static str> {
        Some("paramslider")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|param_slider_event, meta| match param_slider_event {
            SliderEvent::CancelTextInput => {
                self.text_input_active = false;
                cx.set_active(false);

                meta.consume();
            }
            SliderEvent::TextInput(string) => {
                if let Some(normalized_value) = self.param_base.string_to_normalized_value(string) {
                    self.param_base.begin_set_parameter(cx);
                    self.param_base.set_normalized_value(cx, normalized_value);
                    self.param_base.end_set_parameter(cx);
                }

                self.text_input_active = false;

                meta.consume();
            }
        });

        event.map(|window_event: &WindowEvent, meta| match window_event {
            WindowEvent::MouseDown(MouseButton::Left)
            | WindowEvent::MouseTripleClick(MouseButton::Left) => {
                if cx.modifiers().command() {
                    self.param_base.begin_set_parameter(cx);
                    self.param_base
                        .set_normalized_value(cx, self.param_base.default_normalized_value());
                    self.param_base.end_set_parameter(cx);
                } else if !self.text_input_active {
                    self.dragging = true;
                    cx.capture();
                    cx.focus();
                    cx.set_active(true);

                    // When holding down shift while clicking on a parameter we want to granuarly
                    // edit the parameter without jumping to a new value
                    self.param_base.begin_set_parameter(cx);
                    if cx.modifiers().shift() {
                        self.granular_drag_status = Some(GranularDragStatus {
                            starting_x_coordinate: cx.mouse().cursorx,
                            starting_value: self.param_base.unmodulated_normalized_value(),
                        });
                    } else {
                        self.granular_drag_status = None;
                    }
                }
            }
            WindowEvent::MouseDoubleClick(MouseButton::Left) => {
                if cfg!(not(windows)) {
                    self.text_input_active = true;
                }
            }
            WindowEvent::MouseMove(x, _y) => {
                if self.dragging {
                    // If shift is being held then the drag should be more granular instead of
                    // absolute
                    if cx.modifiers().shift() {
                        let granular_drag_status =
                            *self
                                .granular_drag_status
                                .get_or_insert_with(|| GranularDragStatus {
                                    starting_x_coordinate: *x,
                                    starting_value: self.param_base.unmodulated_normalized_value(),
                                });

                        // These positions should be compensated for the DPI scale so it remains
                        // consistent
                        let start_x =
                            remap_current_entity_x_t(cx, granular_drag_status.starting_value);
                        let delta_x = ((*x - granular_drag_status.starting_x_coordinate)
                            * GRANULAR_DRAG_MULTIPLIER)
                            * cx.scale_factor();

                        self.set_normalized_value_drag(
                            cx,
                            remap_current_entity_x_coordinate(cx, start_x + delta_x),
                        );
                    } else {
                        self.granular_drag_status = None;
                        self.set_normalized_value_drag(
                            cx,
                            remap_current_entity_x_coordinate(cx, *x),
                        );
                    }
                }
            }
            WindowEvent::MouseUp(MouseButton::Left) => {
                if self.dragging {
                    self.dragging = false;
                    cx.release();
                    cx.set_active(false);

                    self.param_base.end_set_parameter(cx);

                    meta.consume();
                }
            }
            WindowEvent::MouseScroll(_scroll_x, scroll_y) => {
                // With a regular scroll wheel `scroll_y` will only ever be -1 or 1, but with smooth
                // scrolling trackpads being a thing `scroll_y` could be anything.
                self.scrolled_lines += scroll_y;

                if self.scrolled_lines.abs() >= 1.0 {
                    let use_finer_steps = cx.modifiers().shift();

                    // Scrolling while dragging needs to be taken into account here
                    if !self.dragging {
                        self.param_base.begin_set_parameter(cx);
                    }

                    let mut current_value = self.param_base.unmodulated_normalized_value();

                    while self.scrolled_lines >= 1.0 {
                        current_value = self
                            .param_base
                            .next_normalized_step(current_value, use_finer_steps);
                        self.param_base.set_normalized_value(cx, current_value);
                        self.scrolled_lines -= 1.0;
                    }

                    while self.scrolled_lines <= -1.0 {
                        current_value = self
                            .param_base
                            .previous_normalized_step(current_value, use_finer_steps);
                        self.param_base.set_normalized_value(cx, current_value);
                        self.scrolled_lines += 1.0;
                    }

                    if !self.dragging {
                        self.param_base.end_set_parameter(cx);
                    }
                }

                meta.consume();
            }
            WindowEvent::KeyDown(Code::ArrowRight, _) | WindowEvent::KeyDown(Code::ArrowUp, _) => {
                if self.text_input_active {
                    return;
                }

                self.param_base.begin_set_parameter(cx);
                let current_value = self.param_base.unmodulated_normalized_value();

                let current_value = self
                    .param_base
                    .next_normalized_step(current_value, cx.modifiers().contains(Modifiers::SHIFT));
                self.param_base.set_normalized_value(cx, current_value);

                self.param_base.end_set_parameter(cx);
            }
            WindowEvent::KeyDown(Code::ArrowLeft, _) | WindowEvent::KeyDown(Code::ArrowDown, _) => {
                if self.text_input_active {
                    return;
                }

                self.param_base.begin_set_parameter(cx);
                let current_value = self.param_base.unmodulated_normalized_value();

                let current_value = self.param_base.previous_normalized_step(
                    current_value,
                    cx.modifiers().contains(Modifiers::SHIFT),
                );
                self.param_base.set_normalized_value(cx, current_value);

                self.param_base.end_set_parameter(cx);
            }
            WindowEvent::KeyDown(Code::Enter, _) => {
                self.text_input_active = true;
            }
            _ => {}
        });
    }
}
