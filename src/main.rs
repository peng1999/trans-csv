mod sundown;

use std::ffi::{CStr, CString};

fn main() {
    let s = unsafe {
        let mut b_ptr = sundown::render(CString::new("# Hell$o").unwrap().as_ptr() as *const u8);
        let s = CStr::from_ptr(b_ptr.as_ref().unwrap().data as *const i8)
            .to_str()
            .unwrap()
            .to_owned();
        sundown::bufrelease(b_ptr);
        s
    };
    println!("{}", s);
}
