use obs_sys::obs_output_pause;

pub fn recording_active() -> bool {
    unsafe { obs_output_pause() }
}
