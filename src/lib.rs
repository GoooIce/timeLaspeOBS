// use std::collections::VecDeque;

// use obs_sys::encoder_packet;

// use obs_wrapper::{
//     log::Logger,
//     // Macro for registering modules
//     obs_register_module,
//     // Macro for creating strings
//     obs_string,
//     output::{
//         CreatableOutputContext, EncodedPacketOutput, GetDefaultsOutput, GetNameOutput,
//         GetPropertiesOutput, OutputContext, Outputable, RawVideoOutput, UpdateOutput,
//     },
//     // Everything required for modules
//     prelude::*,
//     properties::{Properties, TextProp, TextType},
//     // Everything required for creating a source
//     source::*,
// };

// use log::info;

// // mod frontend;
// mod audio;

// // #region TestOutput
// struct TestOutput {}

// impl GetNameOutput for TestOutput {
//     fn get_name() -> ObsString {
//         info!("get_name_test_output");
//         obs_string!("Test Output")
//     }
// }

// impl UpdateOutput for TestOutput {
//     fn update(&mut self, settings: &mut DataObj) {
//         info!("update_test_output: {:?}", settings.get_json());
//     }
// }

// impl GetPropertiesOutput for TestOutput {
//     fn get_properties(&mut self) -> Properties {
//         let mut properties_out = Properties::new();
//         properties_out.add::<TextProp>(
//             obs_string!("path"),
//             obs_string!("TestOutput.FilePath"),
//             TextProp::new(TextType::Default),
//         );

//         properties_out
//     }
// }

// impl GetDefaultsOutput for TestOutput {
//     fn get_defaults(settings: &mut DataObj) {
//         info!("get_defaults_test_output");
//     }
// }

// impl EncodedPacketOutput for TestOutput {
//     fn encoded_packet(&mut self, packet: &mut encoder_packet) {
//         info!("encode_packet_test_output");
//     }
// }

// impl Outputable for TestOutput {
//     fn get_id() -> ObsString {
//         info!("get_id_test_output");
//         obs_string!("test_output")
//     }

//     // fn get_type() -> ObsString {}

//     fn create(context: &mut CreatableOutputContext<'_, Self>, output: OutputContext) -> Self {
//         info!("create_test_output");
//         Self {}
//     }
//     fn start(&mut self) -> bool {
//         info!("start_test_output");
//         true
//     }
//     fn stop(&mut self, _ts: u64) {
//         info!("stop_test_output");
//     }
// }

// impl RawVideoOutput for TestOutput {
//     fn raw_video(&mut self, frame: &mut obs_sys::video_data) {
//         info!("raw_test_output")
//     }
// }
// // #endregion

// struct TestModule {
//     context: ModuleContext,
// }

// impl Module for TestModule {
//     fn new(context: ModuleContext) -> Self {
//         Self { context }
//     }

//     fn get_ctx(&self) -> &ModuleContext {
//         &self.context
//     }

//     fn load(&mut self, load_context: &mut LoadContext) -> bool {
//         let _ = Logger::new().init();
//         let source = load_context
//             .create_source_builder::<audio::TestSource>()
//             .enable_get_name()
//             .enable_filter_audio()
//             .build();

//         let output = load_context
//             .create_output_builder::<TestOutput>()
//             .enable_get_name()
//             .build();

//         load_context.register_source(source);
//         load_context.register_output(output);

//         true
//     }

//     fn description() -> ObsString {
//         obs_string!("test module description")
//     }

//     fn name() -> ObsString {
//         obs_string!("Test Module")
//     }

//     fn author() -> ObsString {
//         obs_string!("GoooIce")
//     }

//     fn unload(&mut self) {}

//     fn post_load(&mut self) {}
// }

// obs_register_module!(TestModule);
