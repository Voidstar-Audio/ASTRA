use std::{
    fmt::Pointer,
    sync::{atomic::AtomicU32, Arc},
};

use nih_plug::{editor::Editor, prelude::Enum};
use nih_plug_vizia::{
    create_vizia_editor,
    vizia::{icons::ICON_CHEVRON_DOWN, prelude::*},
    ViziaState, ViziaTheming,
};

use astra::prelude::*;

use crate::ViewsPluginParams;

#[derive(Enum, Clone, Default)]
enum FooEnum {
    #[default]
    Foo,
    Bar,
    Baz,
}

#[derive(Lens)]
struct Data {
    params: Arc<ViewsPluginParams>,
    text: String,
    switch: bool,
    foo: FooEnum,
    dark_mode: bool,
}

enum AppEvent {
    ToggleSwitch,
    ToggleDarkMode,
    SwitchFoo(usize),
}

impl Model for Data {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::ToggleSwitch => {
                self.switch ^= true;
            }
            AppEvent::ToggleDarkMode => {
                self.dark_mode ^= true;
            }
            AppEvent::SwitchFoo(i) => self.foo = FooEnum::from_index(*i),
        });
    }
}

pub(crate) fn create(
    params: Arc<ViewsPluginParams>,
    height: Arc<AtomicU32>,
) -> Option<Box<dyn Editor>> {
    let h = height.clone();
    create_vizia_editor(
        ViziaState::new(move || (600, h.load(std::sync::atomic::Ordering::Relaxed))),
        ViziaTheming::None,
        move |cx, gui| {
            apply_styles(cx);

            Data {
                params: params.clone(),
                text: "Test".to_owned(),
                switch: false,
                foo: Default::default(),
                dark_mode: false,
            }
            .build(cx);

            VStack::new(cx, |cx| {
                HStack::new(cx, |cx| {
                    Image::new(cx, "voidstar_logo.png").size(Pixels(16.0));
                    Label::new(cx, "ASTRA - ALL VIEWS").width(Stretch(1.0));
                    Label::new(cx, "DARK MODE");
                    Switch::new(cx, Data::dark_mode)
                        .on_toggle(|cx| cx.emit(AppEvent::ToggleDarkMode));
                })
                .border_width(Pixels(1.0))
                .space(Pixels(-1.0))
                .z_index(5)
                .child_space(Pixels(16.0))
                .col_between(Pixels(8.0))
                .height(Auto);

                ScrollView::new(cx, 0.0, 0.0, false, true, move |cx| {
                    components(cx, "BUTTON", move |cx| {
                        Button::new(cx, |_| {}, |cx| Label::new(cx, "REGULAR"));
                        Button::new(cx, |_| {}, |cx| Label::new(cx, "PRIMARY")).class("primary");
                        Button::new(cx, |_| {}, |cx| Label::new(cx, "DESTRUCTIVE"))
                            .class("destructive");
                        Button::new(cx, |_| {}, |cx| Label::new(cx, "REGULAR LARGE"))
                            .class("large");
                        Button::new(cx, |_| {}, |cx| Label::new(cx, "PRIMARY LARGE"))
                            .class("primary")
                            .class("large");
                        Button::new(cx, |_| {}, |cx| Label::new(cx, "DESTRUCTIVE LARGE"))
                            .class("destructive")
                            .class("large");
                    });

                    components(cx, "TEXTBOX", |cx| {
                        Textbox::new(cx, Data::text).width(Pixels(128.0));
                    });

                    components(cx, "DROPDOWN", |cx| {
                        Dropdown::new(
                            cx,
                            |cx| {
                                HStack::new(cx, |cx| {
                                    Label::new(cx, "BASIC TEXT")
                                        .width(Stretch(1.0))
                                        .pointer_events(false);
                                    Label::new(cx, ICON_CHEVRON_DOWN).pointer_events(false);
                                })
                            },
                            |cx| {
                                Label::new(cx, "You ever ate\nburgers on a\nwednesdaay")
                                    .left(Pixels(4.0))
                                    .width(Stretch(1.0));
                            },
                        )
                        .width(Pixels(90.0));

                        Dropdown::new(
                            cx,
                            |cx| {
                                HStack::new(cx, |cx| {
                                    Label::new(
                                        cx,
                                        Data::foo
                                            .map(|foo| FooEnum::variants()[foo.clone().to_index()]),
                                    )
                                    .width(Stretch(1.0))
                                    .pointer_events(false);
                                    Label::new(cx, ICON_CHEVRON_DOWN).pointer_events(false);
                                })
                            },
                            |cx| {
                                for (i, variant) in FooEnum::variants().iter().enumerate() {
                                    Label::new(cx, *variant)
                                        .on_press(move |cx| {
                                            cx.emit(AppEvent::SwitchFoo(i));
                                            cx.emit(PopupEvent::Close);
                                        })
                                        .class("option")
                                        .toggle_class(
                                            "selected",
                                            Data::foo.map(move |foo| foo.clone().to_index() == i),
                                        );
                                }
                            },
                        )
                        .width(Pixels(90.0));
                    });

                    components(cx, "SELECTOR", |cx| {
                        Selector::new(cx, Data::foo)
                            .on_toggle(|cx, i| cx.emit(AppEvent::SwitchFoo(i)));
                    });

                    components(cx, "SWITCH", |cx| {
                        Switch::new(cx, Data::switch)
                            .on_toggle(|cx| cx.emit(AppEvent::ToggleSwitch));
                    });

                    components(cx, "CHECKBOX", |cx| {
                        Checkbox::new(cx, Data::switch)
                            .on_toggle(|cx| cx.emit(AppEvent::ToggleSwitch));
                    });

                    components(cx, "RADIO", |cx| {
                        RadioButton::new(cx, Data::switch)
                            .on_select(|cx| cx.emit(AppEvent::ToggleSwitch));
                    });

                    components(cx, "TAG", |cx| {
                        Tag::new(cx, |cx| {
                            Label::new(cx, "Regular");
                        });
                        Tag::new(cx, |cx| {
                            Label::new(cx, "Primary");
                        })
                        .class("primary");
                        Tag::new(cx, |cx| {
                            Label::new(cx, "Red");
                        })
                        .class("bg-red");
                        Tag::new(cx, |cx| {
                            Label::new(cx, "Orange");
                        })
                        .class("bg-orange");
                        Tag::new(cx, |cx| {
                            Label::new(cx, "Yellow");
                        })
                        .class("bg-yellow");
                        Tag::new(cx, |cx| {
                            Label::new(cx, "Green");
                        })
                        .class("bg-green");
                        Tag::new(cx, |cx| {
                            Label::new(cx, "Cyan");
                        })
                        .class("bg-cyan");
                        Tag::new(cx, |cx| {
                            Label::new(cx, "Blue");
                        })
                        .class("bg-blue");
                        Tag::new(cx, |cx| {
                            Label::new(cx, "Violet");
                        })
                        .class("bg-violet");
                        Tag::new(cx, |cx| {
                            Label::new(cx, "Pink");
                        })
                        .class("bg-pink");
                    });

                    components(cx, "PARAMETER DROPDOWN", |cx| {
                        ParamDropdown::new(cx, Data::params, |p| &p.shape).width(Pixels(64.0));
                    });

                    components(cx, "PARAMETER SELECTOR", |cx| {
                        ParamSelector::new(cx, Data::params, |params| &params.shape);
                    });

                    components(cx, "PARAMETER SLIDER", |cx| {
                        ParamSlider::new(cx, Data::params, |p| &p.gain, None).width(Pixels(160.0));
                        ParamSlider::new(
                            cx,
                            Data::params,
                            |p| &p.gain,
                            (0..=16).map(|i| {
                                let pos = i as f32 / 16.0;
                                let value = (-24.0 + 48.0 * pos) as i32;
                                let short = value % 2 != 0;

                                SliderTick {
                                    pos,
                                    label: (!short).then_some(format!("{value:.2}")),
                                    short,
                                }
                            }),
                        )
                        .width(Pixels(160.0));
                    });
                });
            })
            .toggle_class("dark", Data::dark_mode);
        },
    )
}

fn components(cx: &mut Context, name: &str, content: impl Fn(&mut Context)) {
    Label::new(cx, name)
        .class("bg-elevated")
        .width(Stretch(1.0))
        .child_left(Pixels(16.0))
        .child_top(Pixels(8.0))
        .child_bottom(Pixels(8.0));
    HStack::new(cx, content)
        .height(Auto)
        .width(Percentage(100.0))
        .left(Stretch(1.0))
        .right(Stretch(1.0))
        .col_between(Pixels(8.0))
        .child_left(Pixels(16.0))
        .child_top(Pixels(12.0))
        .child_bottom(Pixels(12.0));
}
