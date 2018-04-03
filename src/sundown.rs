use std::ffi::{CStr, CString};

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
mod ffi {
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub struct TextBuf(*mut ffi::buf);

impl TextBuf {
    pub fn as_ref(&self) -> &str {
        unsafe {
        CStr::from_ptr(self.0.as_ref().unwrap().data as *const i8)
            .to_str()
            .unwrap()
        }
    }
}

impl Drop for TextBuf {
    fn drop(&mut self) {
        unsafe { ffi::bufrelease(self.0); }
    }
}

pub fn render(input: &str) -> TextBuf {
    unsafe {
        let mut b_ptr = ffi::render(CString::new(input).unwrap().as_ptr() as *const u8);
        TextBuf(b_ptr)
    }
}
