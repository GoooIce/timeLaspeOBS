// use crate::source::{traits::Sourceable, SourceInfo, SourceInfoBuilder};
// use crate::output::{traits::Outputable, OutputInfo, OutputInfoBuilder};
// use crate::string::ObsString;
use libobs_sys::{
    obs_module_t, obs_output_info, obs_register_output_s, obs_register_source_s, obs_source_info,
    size_t,
};
use std::marker::PhantomData;
