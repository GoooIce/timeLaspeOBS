use std::collections::VecDeque;

use obs_wrapper::{
    log::Logger,
    // Macro for registering modules
    obs_register_module,
    // Macro for creating strings
    obs_string,
    // Everything required for modules
    prelude::*,
    // Everything required for creating a source
    source::*,
};

use log::info;

use dasp::{
    interpolate::linear::Linear,
    signal::{self, interpolate::Converter, Signal},
};

mod frontend;

const RNNOISE_SAMPLE_RATE: f64 = 48000.;
const WAV_COEFFICIENT: f32 = 32767.0;
const FRAME_SIZE_SHIFT: usize = 2;
const FRAME_SIZE: usize = 120 << FRAME_SIZE_SHIFT;

struct TestModule {
    context: ModuleContext,
}

#[derive(Debug)]
struct Output {
    buffer: VecDeque<f32>,
    last_input: (f32, f32),
    last_output: (f32, f32),
}

struct TestSource {
    output: Output,
    input: VecDeque<f32>,
    temp: [f32; FRAME_SIZE],
    temp_out: [f32; FRAME_SIZE],
    sample_rate: f64,
    channels: usize,
}

impl Sourceable for TestSource {
    fn get_id() -> ObsString {
        obs_string!("test_source")
    }
    fn get_type() -> SourceType {
        SourceType::FILTER
    }

    fn create(create: &mut CreatableSourceContext<Self>, _source: SourceContext) -> Self {
        let (sample_rate, channels) =
            create.with_audio(|audio| (audio.output_sample_rate(), audio.output_channels()));

        let _ = Logger::new().init();

        info!("this is a test source create info");

        Self {
            input: VecDeque::with_capacity(FRAME_SIZE * 3),
            output: Output {
                buffer: VecDeque::with_capacity(FRAME_SIZE),
                last_input: (0., 0.),
                last_output: (0., 0.),
            },
            temp: [0.; FRAME_SIZE],
            temp_out: [0.; FRAME_SIZE],
            sample_rate: sample_rate as f64,
            channels,
        }
    }
}

impl GetNameSource for TestSource {
    fn get_name() -> ObsString {
        obs_string!("Test Source")
    }
}

impl UpdateSource for TestSource {
    fn update(&mut self, _settings: &mut DataObj, context: &mut GlobalContext) {
        let sample_rate = context.with_audio(|audio| audio.output_sample_rate());
        self.sample_rate = sample_rate as f64;
    }
}

impl FilterAudioSource for TestSource {
    fn filter_audio(&mut self, audio: &mut audio::AudioDataContext) {
        let data = self;
        // let input_ring_buffer = &mut data.input;
        // let output_state = &mut data.output;

        // let temp = &mut data.temp;
        // let temp_out = &mut data.temp_out;

        // info!(
        //     "input_ring_buffer===========\r\n{:?}\r\n===========output_state===========\r\n{:?}\r\n===========temp===========\r\n{:?}\r\n===========temp_out===========\r\n{:?}\r\n===========",
        //     input_ring_buffer, output_state, temp, temp_out
        // );
        info!("channels: {:?}", data.channels);
        info!("active: {:?}", frontend::recording_active());

        if let Some(base) = audio.get_channel_as_mut_slice(0) {
            for channel in 1..data.channels {
                let buffer = audio
                    .get_channel_as_mut_slice(channel)
                    .expect("Channel count said there was a buffer here.");

                for (output, input) in buffer.iter_mut().zip(base.iter()) {
                    *output = *input;
                }
            }
        }
    }
}

impl Module for TestModule {
    fn new(context: ModuleContext) -> Self {
        Self { context }
    }

    fn get_ctx(&self) -> &ModuleContext {
        &self.context
    }

    fn load(&mut self, load_context: &mut LoadContext) -> bool {
        let source = load_context
            .create_source_builder::<TestSource>()
            .enable_get_name()
            .enable_filter_audio()
            .build();

        load_context.register_source(source);

        true
    }

    fn description() -> ObsString {
        obs_string!("test module description")
    }

    fn name() -> ObsString {
        obs_string!("Test Module")
    }

    fn author() -> ObsString {
        obs_string!("GoooIce")
    }

    fn unload(&mut self) {}

    fn post_load(&mut self) {}
}

obs_register_module!(TestModule);
