use obs_sys::{obs_frontend_recording_active, obs_frontend_recording_pause};

pub fn recording_active() -> bool {
    unsafe { obs_frontend_recording_active() }
}

pub fn recording_pause(inner: bool) -> bool {
    unsafe { obs_frontend_recording_pause(inner) }
}
