use chua::upload_blocking;
use std::ffi::CStr;

#[repr(C)]
pub enum Status {
    Ok = 0,
    Null,
    Invalid,
    Upload,
}

#[no_mangle]
pub extern "C" fn upload(
    url: *const i8,
    path: *const i8,
    chunk_size: usize,
    parallel: usize,
) -> Status {
    if url.is_null() || path.is_null() {
        return Status::Null;
    }

    let url = match unsafe { CStr::from_ptr(url).to_owned() }.into_string() {
        Ok(s) => s,
        Err(_) => return Status::Invalid,
    };

    let path = match unsafe { CStr::from_ptr(path).to_owned() }.into_string() {
        Ok(s) => s,
        Err(_) => return Status::Invalid,
    };

    match upload_blocking(&url, path, chunk_size, parallel) {
        Ok(()) => Status::Ok,
        Err(_) => Status::Upload,
    }
}
