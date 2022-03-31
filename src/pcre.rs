use std::ffi::c_void;
use std::ptr::null;
use std::str::from_utf8;

const ERROR_CODE_NO_ERROR: i32 = 100;

#[link(name = "pcre2-8")]
extern "C" {
    pub fn pcre2_compile_8(pattern: *const u8, length: usize, options: u32, error_code: *mut i32, error_offset: *mut usize, context: *const c_void) -> *const c_void;
    pub fn pcre2_get_error_message_8(errorcode: i32, buffer: *mut u8, bufflen: usize) -> i32;
    pub fn pcre2_match_data_create_from_pattern_8(code: *const c_void, gcontext: *const c_void) -> *mut c_void;
    pub fn pcre2_match_8(code: *const c_void, subject: *const u8, length: usize, startoffset: usize, options: u32, match_data: *mut c_void, mcontext: *const c_void) -> i32;
    pub fn pcre2_get_ovector_pointer_8(match_data: *const c_void) -> *const usize;
}

pub fn pcre_match(pattern: &str, subject: &str) -> Result<Vec<String>, String> {
    unsafe {
        let mut error_code = 0;
        let mut error_offset = 0;
        let re = pcre2_compile_8(pattern.as_ptr(), pattern.len(), 0, &mut error_code, &mut error_offset, null());
        let mut buffer = vec![0; 1024];
        if error_code != ERROR_CODE_NO_ERROR {
            let size = pcre2_get_error_message_8(error_code, buffer.as_mut_ptr(), 1024);
            return Err(from_utf8(&buffer[..size as usize]).unwrap().to_owned());
        }
        let match_data = pcre2_match_data_create_from_pattern_8(re, null());
        let rc = pcre2_match_8(re, subject.as_ptr(), subject.len(), 0, 0, match_data, null());
        if rc < 0 {
            return Ok(vec![]);
        }
        let mut ovector = pcre2_get_ovector_pointer_8(match_data);
        let mut res = Vec::new();
        for _ in 0..rc {
            let v = ovector as *const [usize; 2];
            let (start, end) = ((*v)[0], (*v)[1]);
            res.push(subject[start..end].to_owned());
            let new_address = ovector as u64 + 16;
            ovector = new_address as *const usize;
        }
        Ok(res)
    }
}
