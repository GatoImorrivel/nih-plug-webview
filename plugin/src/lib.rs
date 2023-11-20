use std::{f32::consts::PI, sync::Arc};

pub use nih_plug;
use nih_plug::prelude::*;
use nih_plug::params::Params;
use nih_plug_webview::{create_webview_editor, WebViewState};

#[derive(Debug, Default)]
pub struct WebViewPlugin {
    params: Arc<WebViewParams>
}

#[derive(Params, Debug)]
struct WebViewParams { 
    #[persist = "editor-state"]
    web_state: Arc<WebViewState>,

    #[id = "gozometro"]
    gozometro: FloatParam
}

impl Default for WebViewParams {
    fn default() -> Self {
        Self {
            gozometro: FloatParam::new("gozometro", 0.0, FloatRange::Linear { min: 0.0, max: 100.0 }),
            web_state: Arc::new(WebViewState::new(1260, 848))
        }
    }
}

impl Plugin for WebViewPlugin {
    const NAME: &'static str = "WebViewPlugin";
    const VENDOR: &'static str = "GatoImorrivel";
    const URL: &'static str = env!("CARGO_PKG_HOMEPAGE");
    const EMAIL: &'static str = "gsantos1510@gmail.com";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");   

    // The SIMD version only supports stereo
    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZeroU32::new(2 as u32),
        main_output_channels: NonZeroU32::new(2 as u32),
        ..AudioIOLayout::const_default()
    }];

    const SAMPLE_ACCURATE_AUTOMATION: bool = false;
    const HARD_REALTIME_ONLY: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn editor(&mut self, async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        create_webview_editor(self.params.web_state.clone(), nih_plug_webview::WebViewContent::URL("http://localhost:8080"))
    }

    fn params(&self) -> std::sync::Arc<dyn Params> {
        self.params.clone()
    }

    fn process(
            &mut self,
            buffer: &mut Buffer,
            aux: &mut AuxiliaryBuffers,
            context: &mut impl ProcessContext<Self>,
        ) -> ProcessStatus {
 
        ProcessStatus::Normal
    }
} 

impl Vst3Plugin for WebViewPlugin {
    const VST3_CLASS_ID: [u8; 16] = *b"WebViewGatoPlugs";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
        Vst3SubCategory::Fx,
        Vst3SubCategory::Filter,
        Vst3SubCategory::Stereo,
    ];
}

nih_export_vst3!(WebViewPlugin);