use std::ffi::{CStr, CString};
use bsv::PrivateKey as BSVPrivateKey;
use bsv::PublicKey as BSVPublicKey;
use bsv::ECIESCiphertext;
use bsv::Signature;

#[repr(C)]
pub struct ByteArray {
    data: *mut u8,
    len: usize,
}


// privatekey.to_bytes
#[no_mangle]
pub extern "C" fn privatekey_to_bytes(private_key: *mut BSVPrivateKey) -> ByteArray {
    let private_key = unsafe { &mut *private_key };
    let bytes = private_key.to_bytes();
    let len = bytes.len();
    let out = bytes.into_boxed_slice();
    ByteArray { data: Box::into_raw(out) as *mut u8, len }
}

#[no_mangle]
pub extern "C" fn privatekey_to_hex(private_key: *mut BSVPrivateKey) -> *mut libc::c_char {
    let private_key = unsafe { &mut *private_key };
    let hex = private_key.to_hex();
    let hex_cstr = CString::new(hex).unwrap();
    hex_cstr.into_raw()
}

// privatekey.from_random
#[no_mangle]
pub extern "C" fn privatekey_from_random() -> *mut BSVPrivateKey {
    let private_key = BSVPrivateKey::from_random();
    Box::into_raw(Box::new(private_key))
}

// pub fn get_point(&self) -> Vec<u8> {
//         self.0.get_point()
//     }
#[no_mangle]
pub extern "C" fn privatekey_get_point(private_key: *mut BSVPrivateKey) -> ByteArray {
    let private_key = unsafe { &mut *private_key };
    let bytes = private_key.get_point();
    let len = bytes.len();
    let out = bytes.into_boxed_slice();
    ByteArray { data: Box::into_raw(out) as *mut u8, len }
}

// pub fn compress_public_key(&self, should_compress: bool) -> PrivateKey {
//         PrivateKey(self.0.compress_public_key(should_compress))
//     }
#[no_mangle]
pub extern "C" fn privatekey_compress_public_key(private_key: *mut BSVPrivateKey, should_compress: bool) -> *mut BSVPrivateKey {
    let private_key = unsafe { &mut *private_key };
    let private_key = private_key.compress_public_key(should_compress);
    Box::into_raw(Box::new(private_key))
}

// pub fn from_wif(wif_string: &str) -> Result<PrivateKey, wasm_bindgen::JsError> {
//         Ok(PrivateKey(BSVPrivateKey::from_wif(wif_string)?))
//     }
#[no_mangle]
pub extern "C" fn privatekey_from_wif(wif: *const libc::c_char) -> *mut BSVPrivateKey {
    let wif_cstr = unsafe { CStr::from_ptr(wif) };
    let wif = wif_cstr.to_str().unwrap();
    let private_key = BSVPrivateKey::from_wif(wif).unwrap();
    Box::into_raw(Box::new(private_key))
}


#[no_mangle]
pub extern "C" fn privatekey_from_hex(hex: *const libc::c_char) -> *mut BSVPrivateKey {
    let hex_cstr = unsafe { CStr::from_ptr(hex) };
    let hex = hex_cstr.to_str().unwrap();
    let private_key = BSVPrivateKey::from_hex(hex).unwrap();
    Box::into_raw(Box::new(private_key))
}

// pub fn sign_message(&self, msg: &[u8]) -> Result<Signature, wasm_bindgen::JsError> {
//         Ok(Signature(self.0.sign_message(msg)?))
//     }
#[no_mangle]
pub extern "C" fn privatekey_sign_message(private_key: *mut BSVPrivateKey, msg: *const u8, msg_len: usize) -> *mut Signature {
    let private_key = unsafe { &mut *private_key };
    let msg = unsafe { std::slice::from_raw_parts(msg, msg_len) };
    let signature = private_key.sign_message(msg).unwrap();
    Box::into_raw(Box::new(signature))
}

#[no_mangle]
pub extern "C" fn privatekey_to_wif(private_key: *mut BSVPrivateKey) -> *mut libc::c_char {
    let private_key = unsafe { &mut *private_key };
    let wif = private_key.to_wif().unwrap();
    let wif_cstr = CString::new(wif).unwrap();
    wif_cstr.into_raw()
}

// pub fn from_bytes(bytes: &[u8]) -> Result<PrivateKey, wasm_bindgen::JsError> {
//         Ok(PrivateKey(BSVPrivateKey::from_bytes(bytes)?))
//     }
#[no_mangle]
pub extern "C" fn privatekey_from_bytes(bytes: *const u8, bytes_len: usize) -> *mut BSVPrivateKey {
    let bytes = unsafe { std::slice::from_raw_parts(bytes, bytes_len) };
    let private_key = BSVPrivateKey::from_bytes(bytes).unwrap();
    Box::into_raw(Box::new(private_key))
}

// pub fn to_public_key(&self) -> Result<PublicKey, wasm_bindgen::JsError> {
//         Ok(PublicKey(self.0.to_public_key()?))
//     }
#[no_mangle]
pub extern "C" fn privatekey_to_public_key(private_key: *mut BSVPrivateKey) -> *mut BSVPublicKey {
    let private_key = unsafe { &mut *private_key };
    let public_key = private_key.to_public_key().unwrap();
    Box::into_raw(Box::new(public_key))
}

// pub fn encrypt_message(&self, message: &[u8]) -> Result<ECIESCiphertext, wasm_bindgen::JsError> {
//         Ok(ECIESCiphertext(self.0.encrypt_message(message)?))
//     }
#[no_mangle]
pub extern "C" fn privatekey_encrypt_message(private_key: *mut BSVPrivateKey, message: *const u8, message_len: usize) -> *mut ECIESCiphertext {
    let private_key = unsafe { &mut *private_key };
    let message = unsafe { std::slice::from_raw_parts(message, message_len) };
    let ciphertext = private_key.encrypt_message(message).unwrap();
    Box::into_raw(Box::new(ciphertext))
}

// pub fn decrypt_message(&self, ciphertext: &ECIESCiphertext, sender_pub_key: &PublicKey) -> Result<Vec<u8>, wasm_bindgen::JsError> {
//         Ok(self.0.decrypt_message(&ciphertext.0, &sender_pub_key.0)?)
//     }
#[no_mangle]
pub extern "C" fn privatekey_decrypt_message(private_key: *mut BSVPrivateKey, ciphertext: *mut ECIESCiphertext, sender_pub_key: *mut BSVPublicKey) -> ByteArray {
    let private_key = unsafe { &mut *private_key };
    let ciphertext = unsafe { Box::from_raw(ciphertext) };
    let sender_pub_key = unsafe { Box::from_raw(sender_pub_key) };
    let bytes = private_key.decrypt_message(&ciphertext, &sender_pub_key).unwrap();
    let len = bytes.len();
    let out = bytes.into_boxed_slice();
    ByteArray { data: Box::into_raw(out) as *mut u8, len }
}










