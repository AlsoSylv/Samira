use tokio::runtime::Runtime;

pub struct RT {
    pub rt: Runtime,
}

impl RT {
    fn new() -> Self {
        let rt = Runtime::new().expect("pain");
        RT { rt }
    }
}

/// Creates a new opaque pointer to a tokio runtime
#[no_mangle]
pub extern "C" fn new_rt() -> *mut RT {
    Box::into_raw(Box::new(RT::new()))
}

/// Drops the tokio runtime, do not pass null
#[no_mangle]
pub unsafe extern "C" fn drop_rt(rt: *mut RT) {
    drop(Box::from_raw(rt));
}