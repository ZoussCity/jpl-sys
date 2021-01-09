#[test]
fn check_linked() {
    unsafe {
        assert_eq!(jpl_sys::jpl_init_error_code(), jpl_sys::JPL_INIT_NOT_CALLED);
    }
}
