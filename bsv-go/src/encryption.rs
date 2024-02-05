use bsv::AESAlgorithms as BSVAESAlgorithms;
use bsv::AES as BSVAES;

// pub fn encrypt(key: &[u8], iv: &[u8], message: &[u8], algo: AESAlgorithms) -> Result<Vec<u8>, wasm_bindgen::JsError> {
//     Ok(BSVAES::encrypt(key, iv, message, algo.into())?)
// }
#[no_mangle]
pub extern "C" fn aes_encrypt(key: *const u8, key_len: usize, iv: *const u8, iv_len: usize, message: *const u8, message_len: usize, algo: AESAlgorithms) -> ByteArray {
    let key = unsafe { std::slice::from_raw_parts(key, key_len) };
    let iv = unsafe { std::slice::from_raw_parts(iv, iv_len) };
    let message = unsafe { std::slice::from_raw_parts(message, message_len) };
    let bytes = BSVAES::encrypt(key, iv, message, algo.into()).unwrap();
    let len = bytes.len();
    let out = bytes.into_boxed_slice();
    ByteArray { data: Box::into_raw(out) as *mut u8, len }
}

// pub fn decrypt(key: &[u8], iv: &[u8], message: &[u8], algo: AESAlgorithms) -> Result<Vec<u8>, wasm_bindgen::JsError> {
//     Ok(BSVAES::decrypt(key, iv, message, algo.into())?)
// }
#[no_mangle]
pub extern "C" fn aes_decrypt(key: *const u8, key_len: usize, iv: *const u8, iv_len: usize, message: *const u8, message_len: usize, algo: AESAlgorithms) -> ByteArray {
    let key = unsafe { std::slice::from_raw_parts(key, key_len) };
    let iv = unsafe { std::slice::from_raw_parts(iv, iv_len) };
    let message = unsafe { std::slice::from_raw_parts(message, message_len) };
    let bytes = BSVAES::decrypt(key, iv, message, algo.into()).unwrap();
    let len = bytes.len();
    let out = bytes.into_boxed_slice();
    ByteArray { data: Box::into_raw(out) as *mut u8, len }
}