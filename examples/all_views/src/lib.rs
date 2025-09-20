mod editor;

use nih_plug::{prelude::*, util::db_to_gain};
use nih_plug_vizia::ViziaState;
use std::sync::{atomic::AtomicU32, Arc};

pub struct ViewsPlugin {
    params: Arc<ViewsPluginParams>,
}

#[derive(Enum, PartialEq)]
enum Waveshape {
    Sine,
    Saw,
    Square,
}

#[derive(Params)]
struct ViewsPluginParams {
    #[id = "gain"]
    pub gain: FloatParam,
    #[id = "shape"]
    pub shape: EnumParam<Waveshape>,
    #[persist = "editor-height"]
    height: Arc<AtomicU32>,
}

impl Default for ViewsPlugin {
    fn default() -> Self {
        Self {
            params: Arc::new(ViewsPluginParams::default()),
        }
    }
}

impl Default for ViewsPluginParams {
    fn default() -> Self {
        Self {
            gain: FloatParam::new(
                "Gain",
                0.0,
                FloatRange::Linear {
                    min: -24.0,
                    max: 24.0,
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_rounded(2)),
            shape: EnumParam::new("Waveshape", Waveshape::Sine),
            height: Arc::new(700.into()),
        }
    }
}

impl Plugin for ViewsPlugin {
    const NAME: &'static str = "Astra \"All Views\" Demo";
    const VENDOR: &'static str = "Voidstar Audio";
    const URL: &'static str = env!("CARGO_PKG_HOMEPAGE");
    const EMAIL: &'static str = "exa04@pm.me";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZeroU32::new(2),
        main_output_channels: NonZeroU32::new(2),

        aux_input_ports: &[],
        aux_output_ports: &[],

        names: PortNames::const_default(),
    }];

    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::None;

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        _buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        true
    }

    fn reset(&mut self) {}

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        for channel_samples in buffer.iter_samples() {
            let gain = db_to_gain(self.params.gain.smoothed.next());

            for sample in channel_samples {
                *sample *= gain;
            }
        }

        ProcessStatus::Normal
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        editor::create(self.params.clone(), self.params.height.clone())
    }
}

impl ClapPlugin for ViewsPlugin {
    const CLAP_ID: &'static str = "com.voidstar-audio.astra-all-views";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("All views");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;

    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::AudioEffect, ClapFeature::Stereo];
}

impl Vst3Plugin for ViewsPlugin {
    const VST3_CLASS_ID: [u8; 16] = *b"VS-0000_AstraAll";

    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Dynamics];
}

nih_export_clap!(ViewsPlugin);
nih_export_vst3!(ViewsPlugin);
