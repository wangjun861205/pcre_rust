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
    fn pcre2_match_data_create_from_pattern_8(
        code: *const c_void,
        gcontext: *const c_void,
    ) -> *mut c_void;
    fn pcre2_match_8(
        code: *const c_void,
        subject: *const u8,
        length: usize,
        startoffset: usize,
        options: u32,
        match_data: *mut c_void,
        mcontext: *const c_void,
    ) -> i32;
    fn pcre2_get_ovector_pointer_8(match_data: *const c_void) -> *const usize;
}

fn main() {
    unsafe {
        let mut error_code = 0;
        let mut error_offset = 0;
        let pattern = r"(\w+), (\w+)";
        let re = pcre2_compile_8(
            pattern.as_ptr(),
            pattern.len(),
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
        println!("error: {}", from_utf8(&buffer).unwrap());
        let match_data = pcre2_match_data_create_from_pattern_8(re, null());
        let subject = "hello, world";
        let rc = pcre2_match_8(
            re,
            subject.as_ptr(),
            subject.len(),
            0,
            0,
            match_data,
            null(),
        );
        println!("rc: {}", rc);
        let mut ovector = pcre2_get_ovector_pointer_8(match_data);
        let v = ovector as *const [usize; 4];
        println!("start: {}, end: {}", (*v)[0], (*v)[1]);
        println!("start: {}, end: {}", (*v)[2], (*v)[3]);
        for i in 0..rc {
            println!("the {}th ovector: {:?}", i, ovector);
            let v = ovector as *const [usize; 2];
            println!("start: {}, end: {}", (*v)[0], (*v)[1]);
            let new_address = ovector as u64 + 16;
            ovector = new_address as *const usize;
        }
    }
}
