use bsv::Script as BSVScript;
use std::ffi::CStr;
use std::ffi::CString;

#[repr(C)]
pub struct Error {
    message: *mut libc::c_char,
}

#[repr(C)]
pub struct ByteArray {
    data: *mut u8,
    len: usize,
}

impl From<Vec<u8>> for ByteArray {
    fn from(vec: Vec<u8>) -> Self {
        let len = vec.len();
        let data = vec.as_ptr() as *mut u8;
        std::mem::forget(vec);
        Self { data, len }
    }
}

fn set_error_message(error: *mut Error, message: String) {
    unsafe {
        (*error).message = CString::new(message).unwrap().into_raw();
    }
}

// pub fn to_asm_string(&self) -> String {
//     BSVScript::to_asm_string(&self.0)
// }
#[no_mangle]
pub extern "C" fn script_to_asm_string(
    script: *mut BSVScript,
    error: *mut Error,
) -> *mut libc::c_char {
    let script = unsafe { &mut *script };
    let string = BSVScript::to_asm_string(&script);
    let c_str = match CString::new(string) {
        Ok(s) => s,
        Err(e) => {
            set_error_message(error, e.to_string());
            return std::ptr::null_mut();
        }
    };
    c_str.into_raw()
}

// pub fn to_extended_asm_string(&self) -> String {
//     BSVScript::to_extended_asm_string(&self.0)
// }
#[no_mangle]
pub extern "C" fn script_to_extended_asm_string(
    script: *mut BSVScript,
    error: *mut Error,
) -> *mut libc::c_char {
    let script = unsafe { &mut *script };
    let string = BSVScript::to_extended_asm_string(&script);
    let c_str = match CString::new(string) {
        Ok(s) => s,
        Err(e) => {
            set_error_message(error, e.to_string());
            return std::ptr::null_mut();
        }
    };
    c_str.into_raw()
}

// pub fn from_hex(hex: &str) -> Result<Script, wasm_bindgen::JsError> {
//     Ok(Script(BSVScript::from_hex(hex)?))
// }
#[no_mangle]
pub extern "C" fn script_from_hex(hex: *mut libc::c_char, error: *mut Error) -> *mut BSVScript {
    let hex = unsafe { CStr::from_ptr(hex) };
    let hex = match hex.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_error_message(error, e.to_string());
            return std::ptr::null_mut();
        }
    };
    let script = match BSVScript::from_hex(hex) {
        Ok(s) => s,
        Err(e) => {
            set_error_message(error, e.to_string());
            return std::ptr::null_mut();
        }
    };
    Box::into_raw(Box::new(script))
}

// pub fn from_bytes(bytes: &[u8]) -> Result<Script, wasm_bindgen::JsError> {
//     Ok(Script(BSVScript::from_bytes(bytes)?))
// }
#[no_mangle]
pub extern "C" fn script_from_bytes(
    bytes: *mut u8,
    len: usize,
    error: *mut Error,
) -> *mut BSVScript {
    let bytes = unsafe { std::slice::from_raw_parts(bytes, len) };
    let script = match BSVScript::from_bytes(bytes) {
        Ok(s) => s,
        Err(e) => {
            set_error_message(error, e.to_string());
            return std::ptr::null_mut();
        }
    };
    Box::into_raw(Box::new(script))
}

// pub fn from_asm_string(asm_string: &str) -> Result<Script, wasm_bindgen::JsError> {
//     Ok(Script(BSVScript::from_asm_string(asm_string)?))
// }
#[no_mangle]
pub extern "C" fn script_from_asm_string(
    asm_string: *mut libc::c_char,
    error: *mut Error,
) -> *mut BSVScript {
    let asm_string = unsafe { CStr::from_ptr(asm_string) };
    let asm_string = match asm_string.to_str() {
        Ok(s) => s,
        Err(e) => {
            set_error_message(error, e.to_string());
            return std::ptr::null_mut();
        }
    };
    let script = match BSVScript::from_asm_string(asm_string) {
        Ok(s) => s,
        Err(e) => {
            set_error_message(error, e.to_string());
            return std::ptr::null_mut();
        }
    };
    Box::into_raw(Box::new(script))
}

// pub fn encode_pushdata(data_bytes: &[u8]) -> Result<Vec<u8>, wasm_bindgen::JsError> {
//     Ok(BSVScript::encode_pushdata(data_bytes)?)
// }
#[no_mangle]
pub extern "C" fn script_encode_pushdata(
    data_bytes: *mut u8,
    len: usize,
    error: *mut Error,
) -> ByteArray {
    let data_bytes = unsafe { std::slice::from_raw_parts(data_bytes, len) };
    let bytes = match BSVScript::encode_pushdata(data_bytes) {
        Ok(s) => s,
        Err(e) => {
            set_error_message(error, e.to_string());
            return ByteArray {
                data: std::ptr::null_mut(),
                len: 0,
            };
        }
    };
    let len = bytes.len();
    let out = bytes.into_boxed_slice();
    ByteArray {
        data: Box::into_raw(out) as *mut u8,
        len,
    }
}

// pub fn get_pushdata_bytes(length: usize) -> Result<Vec<u8>, wasm_bindgen::JsError> {
//     Ok(BSVScript::get_pushdata_prefix_bytes(length)?)
// }
#[no_mangle]
pub extern "C" fn script_get_pushdata_bytes(length: usize, error: *mut Error) -> ByteArray {
    let bytes = match BSVScript::get_pushdata_prefix_bytes(length) {
        Ok(s) => s,
        Err(e) => {
            set_error_message(error, e.to_string());
            return ByteArray {
                data: std::ptr::null_mut(),
                len: 0,
            };
        }
    };
    let len = bytes.len();
    let out = bytes.into_boxed_slice();
    ByteArray {
        data: Box::into_raw(out) as *mut u8,
        len,
    }
}

// pub fn to_script_bits(&self) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsError> {
//     Ok(serde_wasm_bindgen::to_value(&self.0)?)
// }
#[no_mangle]
pub extern "C" fn script_to_script_bits(
    script: *mut BSVScript,
    error: *mut Error,
) -> *mut libc::c_char {
    let script = unsafe { &mut *script };
    let json = match serde_json::to_string(&script) {
        Ok(s) => s,
        Err(e) => {
            set_error_message(error, e.to_string());
            return std::ptr::null_mut();
        }
    };
    let c_string = match CString::new(json) {
        Ok(s) => s,
        Err(e) => {
            set_error_message(error, e.to_string());
            return std::ptr::null_mut();
        }
    };
    c_string.into_raw()
}

// pub fn to_bytes(&self) -> Vec<u8> {
//     BSVScript::to_bytes(&self.0)
// }
#[no_mangle]
pub extern "C" fn script_to_bytes(script: *mut BSVScript) -> ByteArray {
    let script = unsafe { &mut *script };
    let bytes = BSVScript::to_bytes(&script);
    let len = bytes.len();
    let out = bytes.into_boxed_slice();
    ByteArray {
        data: Box::into_raw(out) as *mut u8,
        len,
    }
}

// pub fn get_script_length(&self) -> usize {
//     BSVScript::get_script_length(&self.0)
// }
#[no_mangle]
pub extern "C" fn script_get_script_length(script: *mut BSVScript) -> usize {
    let script = unsafe { &mut *script };
    BSVScript::get_script_length(&script)
}

// pub fn to_hex(&self) -> String {
//     BSVScript::to_hex(&self.0)
// }
#[no_mangle]
pub extern "C" fn script_to_hex(script: *mut BSVScript, error: *mut Error) -> *mut libc::c_char {
    let script = unsafe { &mut *script };
    let hex = BSVScript::to_hex(&script);
    let c_str = match CString::new(hex) {
        Ok(s) => s,
        Err(e) => {
            set_error_message(error, e.to_string());
            return std::ptr::null_mut();
        }
    };
    c_str.into_raw()
}

// pub fn remove_codeseparators(&mut self) {
//     BSVScript::remove_codeseparators(&mut self.0)
// }
#[no_mangle]
pub extern "C" fn script_remove_codeseparators(script: *mut BSVScript) {
    let script = unsafe { &mut *script };
    BSVScript::remove_codeseparators(script);
}
