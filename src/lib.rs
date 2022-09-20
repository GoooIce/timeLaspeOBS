use libobs_sys::{
    self, _bindgen_ty_1, obs_module_t, LIBOBS_API_MAJOR_VER, LIBOBS_API_MINOR_VER,
    LIBOBS_API_PATCH_VER,
};
use libobs_wrapper::log::Logger;
use log::info;

// static mut OBS_MODULE: Option<$t> = None;
pub static mut OBS_MODULE_POINTER: Option<*mut obs_module_t> = None;

#[no_mangle]
pub unsafe extern "C" fn obs_module_load() -> bool {
    let _ = Logger::new().init();
    info!("test obs module load");

    true
}

#[no_mangle]
pub unsafe extern "C" fn obs_module_unload() {
    // let _ = Logger::new().init();
    info!("test obs module unload");
}

#[no_mangle]
pub unsafe extern "C" fn obs_module_set_pointer(raw: *mut obs_module_t) {
    OBS_MODULE_POINTER = Some(raw)
}

#[no_mangle]
pub unsafe extern "C" fn obs_module_ver() -> u32 {
    (LIBOBS_API_MAJOR_VER << 24) | (LIBOBS_API_MINOR_VER << 16) | LIBOBS_API_PATCH_VER
}
