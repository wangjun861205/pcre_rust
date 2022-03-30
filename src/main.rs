use std::ffi::c_void;
use std::ptr::null;
use std::str::from_utf8;

#[link(name = "pcre2-8")]
extern "C" {
    fn pcre2_compile_8(
        pattern: *const u8,
        length: usize,
        options: u32,
        error_code: *mut i32,
        error_offset: *mut usize,
        context: *const c_void,
    ) -> *const c_void;
    fn pcre2_get_error_message_8(errorcode: i32, buffer: *mut u8, bufflen: usize) -> i32;
}

fn main() {
    unsafe {
        let mut error_code = 0;
        let mut error_offset = 0;
        let pattern = ".*";
        let re = pcre2_compile_8(
            pattern.as_ptr(),
            10,
            0,
            &mut error_code,
            &mut error_offset,
            null(),
        );
        let mut buffer = vec![0; 1024];
        pcre2_get_error_message_8(error_code, buffer.as_mut_ptr(), 1024);
        println!(
            "re: {:?}, error code: {}, error_offset: {}",
            re, error_code, error_offset
        );
        println!("error: {}", from_utf8(&buffer).unwrap())
    }
}
