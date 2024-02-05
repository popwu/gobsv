use bsv::Script as BSVScript;
use std::os::raw::c_char;
use std::ffi::CString;

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

// pub fn to_asm_string(&self) -> String {
//     BSVScript::to_asm_string(&self.0)
// }
#[no_mangle]
pub extern "C" fn script_to_asm_string(script: *mut BSVScript) -> *mut libc::c_char {
    let script = unsafe { &mut *script };
    let string = BSVScript::to_asm_string(&script);
    let c_str = CString::new(string).unwrap();
    c_str.into_raw()
}

// pub fn to_extended_asm_string(&self) -> String {
//     BSVScript::to_extended_asm_string(&self.0)
// }
#[no_mangle]
pub extern "C" fn script_to_extended_asm_string(script: *mut BSVScript) -> *mut libc::c_char {
    let script = unsafe { &mut *script };
    let string = BSVScript::to_extended_asm_string(&script);
    let c_str = CString::new(string).unwrap();
    c_str.into_raw()
}

// pub fn from_hex(hex: &str) -> Result<Script, wasm_bindgen::JsError> {
//     Ok(Script(BSVScript::from_hex(hex)?))
// }
#[no_mangle]
pub extern "C" fn script_from_hex(hex: *mut libc::c_char) -> *mut BSVScript {
    let hex = unsafe { CStr::from_ptr(hex) };
    let hex = hex.to_str().unwrap();
    let script = BSVScript::from_hex(hex).unwrap();
    Box::into_raw(Box::new(script))
}

// pub fn from_bytes(bytes: &[u8]) -> Result<Script, wasm_bindgen::JsError> {
//     Ok(Script(BSVScript::from_bytes(bytes)?))
// }
#[no_mangle]
pub extern "C" fn script_from_bytes(bytes: *mut u8, len: usize) -> *mut BSVScript {
    let bytes = unsafe { std::slice::from_raw_parts(bytes, len) };
    let script = BSVScript::from_bytes(bytes).unwrap();
    Box::into_raw(Box::new(script))
}

// pub fn from_asm_string(asm_string: &str) -> Result<Script, wasm_bindgen::JsError> {
//     Ok(Script(BSVScript::from_asm_string(asm_string)?))
// }
#[no_mangle]
pub extern "C" fn script_from_asm_string(asm_string: *mut libc::c_char) -> *mut BSVScript {
    let asm_string = unsafe { CStr::from_ptr(asm_string) };
    let asm_string = asm_string.to_str().unwrap();
    let script = BSVScript::from_asm_string(asm_string).unwrap();
    Box::into_raw(Box::new(script))
}

// pub fn encode_pushdata(data_bytes: &[u8]) -> Result<Vec<u8>, wasm_bindgen::JsError> {
//     Ok(BSVScript::encode_pushdata(data_bytes)?)
// }
#[no_mangle]
pub extern "C" fn script_encode_pushdata(data_bytes: *mut u8, len: usize) -> ByteArray {
    let data_bytes = unsafe { std::slice::from_raw_parts(data_bytes, len) };
    let bytes = BSVScript::encode_pushdata(data_bytes).unwrap();
    let len = bytes.len();
    let out = bytes.into_boxed_slice();
    ByteArray { data: Box::into_raw(out) as *mut u8, len }
}

// pub fn get_pushdata_bytes(length: usize) -> Result<Vec<u8>, wasm_bindgen::JsError> {
//     Ok(BSVScript::get_pushdata_prefix_bytes(length)?)
// }
#[no_mangle]
pub extern "C" fn script_get_pushdata_bytes(length: usize) -> ByteArray {
    let bytes = BSVScript::get_pushdata_prefix_bytes(length).unwrap();
    let len = bytes.len();
    let out = bytes.into_boxed_slice();
    ByteArray { data: Box::into_raw(out) as *mut u8, len }
}

// pub fn to_script_bits(&self) -> Result<wasm_bindgen::JsValue, wasm_bindgen::JsError> {
//     Ok(serde_wasm_bindgen::to_value(&self.0)?)
// }
#[no_mangle]
pub extern "C" fn script_to_script_bits(script: *mut BSVScript) -> *mut c_char {
    let script = unsafe { &mut *script };
    let json = serde_json::to_string(&script).unwrap();
    let c_string = CString::new(json).unwrap();
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
    ByteArray { data: Box::into_raw(out) as *mut u8, len }
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
pub extern "C" fn script_to_hex(script: *mut BSVScript) -> *mut libc::c_char {
    let script = unsafe { &mut *script };
    let hex = BSVScript::to_hex(&script);
    let c_str = CString::new(hex).unwrap();
    c_str.into_raw()
}

// pub fn remove_codeseparators(&mut self) {
//     BSVScript::remove_codeseparators(&mut self.0)
// }
#[no_mangle]
pub extern "C" fn script_remove_codeseparators(script: *mut BSVScript) {
    let script = unsafe { &mut *script };
    BSVScript::remove_codeseparators(&mut script)
}