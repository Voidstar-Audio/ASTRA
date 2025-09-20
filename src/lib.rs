pub mod param_dropdown;
pub mod param_selector;
pub mod param_slider;
pub mod param_switch;
pub mod selector;
pub mod tag;

use nih_plug_vizia::vizia::{image, prelude::*};

pub mod prelude {
    pub use crate::{
        apply_styles, basics::*, param_dropdown::*, param_selector::*, param_slider::*,
        param_switch::*, selector::*, tag::*,
    };
}

pub fn apply_styles(cx: &mut Context) -> Result<(), std::io::Error> {
    cx.add_font_mem(include_bytes!("../static/VoidstarMono-Regular.otf"));
    cx.load_image(
        "voidstar_logo.png",
        image::load_from_memory_with_format(
            include_bytes!("../static/voidstar_logo.png"),
            image::ImageFormat::Png,
        )
        .unwrap(),
        ImageRetentionPolicy::DropWhenUnusedForOneFrame,
    );
    cx.load_image(
        "chevron_down.png",
        image::load_from_memory_with_format(
            include_bytes!("../static/chevron_down.png"),
            image::ImageFormat::Png,
        )
        .unwrap(),
        ImageRetentionPolicy::DropWhenUnusedForOneFrame,
    );
    cx.add_stylesheet(grass::include!("static/styles.scss"))
}

pub mod basics {
    use nih_plug_vizia::vizia::prelude::*;
    pub fn hdivider(cx: &mut Context) -> Handle<Element> {
        Element::new(cx).class("divider").class("horizontal")
    }
    pub fn vdivider(cx: &mut Context) -> Handle<Element> {
        Element::new(cx).class("divider").class("vertical")
    }
}
