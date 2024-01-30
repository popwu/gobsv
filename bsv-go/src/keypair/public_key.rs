use std::ffi::{CStr, CString};
use bsv::PrivateKey as BSVPrivateKey;
use bsv::PublicKey as BSVPublicKey;
use bsv::ECIESCiphertext;
use bsv::Signature;
use bsv::P2PKHAddress;

#[repr(C)]
pub struct ByteArray {
    data: *mut u8,
    len: usize,
}

// pub fn to_address(&self) -> Result<P2PKHAddress, wasm_bindgen::JsError> {
//         Ok(P2PKHAddress(self.0.to_p2pkh_address()?))
//     }
#[no_mangle]
pub extern "C" fn publickey_to_address(public_key: *mut BSVPublicKey) -> *mut P2PKHAddress {
    let public_key = unsafe { &mut *public_key };
    let address = public_key.to_p2pkh_address().unwrap();
    Box::into_raw(Box::new(address))
}

// pub fn from_hex(hex_str: &str) -> Result<PublicKey, wasm_bindgen::JsError> {
//         Ok(PublicKey(BSVPublicKey::from_hex(hex_str)?))
//     }
#[no_mangle]
pub extern "C" fn publickey_from_hex(hex_str: *mut libc::c_char) -> *mut BSVPublicKey {
    let hex_str = unsafe { CStr::from_ptr(hex_str) };
    let hex_str = hex_str.to_str().unwrap();
    let public_key = BSVPublicKey::from_hex(hex_str).unwrap();
    Box::into_raw(Box::new(public_key))
}

// pub fn from_bytes(bytes: &[u8]) -> Result<PublicKey, wasm_bindgen::JsError> {
//     Ok(PublicKey(BSVPublicKey::from_bytes(bytes)?))
// }
#[no_mangle]
pub extern "C" fn publickey_from_bytes(bytes: *mut u8, len: usize) -> *mut BSVPublicKey {
    let bytes = unsafe { std::slice::from_raw_parts(bytes, len) };
    let public_key = BSVPublicKey::from_bytes(bytes).unwrap();
    Box::into_raw(Box::new(public_key))
}

// pub fn to_bytes(&self) -> Result<Vec<u8>, wasm_bindgen::JsError> {
//     Ok(self.0.to_bytes()?)
// }
#[no_mangle]
pub extern "C" fn publickey_to_bytes(public_key: *mut BSVPublicKey) -> ByteArray {
    let public_key = unsafe { &mut *public_key };
    let bytes = public_key.to_bytes();
    let bytes = match bytes {
        Ok(bytes) => bytes,
        Err(_) => return ByteArray { data: std::ptr::null_mut(), len: 0 },
    };
    let len = bytes.len();
    let out = bytes.into_boxed_slice();
    ByteArray { data: Box::into_raw(out) as *mut u8, len }
}

// pub fn to_hex(&self) -> Result<String, wasm_bindgen::JsError> {
//     Ok(self.0.to_hex()?)
// }
#[no_mangle]
pub extern "C" fn publickey_to_hex(public_key: *mut BSVPublicKey) -> *mut libc::c_char {
    let public_key = unsafe { &mut *public_key };
    let hex = public_key.to_hex();
    let hex = match hex {
        Ok(hex) => hex,
        Err(_) => return std::ptr::null_mut(),  // or handle the error in another way
    };
    let hex_cstr = CString::new(hex).unwrap();
    hex_cstr.into_raw()
}

// pub fn from_private_key(priv_key: &PrivateKey) -> PublicKey {
//     PublicKey(BSVPublicKey::from_private_key(&priv_key.0))
// }
#[no_mangle]
pub extern "C" fn publickey_from_private_key(private_key: *mut BSVPrivateKey) -> *mut BSVPublicKey {
    let private_key = unsafe { &mut *private_key };
    let public_key = BSVPublicKey::from_private_key(&private_key);
    Box::into_raw(Box::new(public_key))
}

// pub fn verify_message(&self, message: &[u8], signature: &Signature) -> Result<bool, wasm_bindgen::JsError> {
//     Ok(self.0.verify_message(message, &signature.0)?)
// }
#[no_mangle]
pub extern "C" fn publickey_verify_message(public_key: *mut BSVPublicKey, message: *const u8, message_len: usize, signature: *mut Signature) -> bool {
    let public_key = unsafe { &mut *public_key };
    let message = unsafe { std::slice::from_raw_parts(message, message_len) };
    let signature = unsafe { &mut *signature };
    public_key.verify_message(message, &signature).unwrap()
}

// pub fn to_p2pkh_address(&self) -> Result<P2PKHAddress, wasm_bindgen::JsError> {
//     Ok(P2PKHAddress(self.0.to_p2pkh_address()?))
// }
#[no_mangle]
pub extern "C" fn publickey_to_p2pkh_address(public_key: *mut BSVPublicKey) -> *mut P2PKHAddress {
    let public_key = unsafe { &mut *public_key };
    let address = public_key.to_p2pkh_address().unwrap();
    Box::into_raw(Box::new(address))
}

// pub fn to_compressed(&self) -> Result<PublicKey, wasm_bindgen::JsError> {
//     Ok(PublicKey(self.0.to_compressed()?))
// }
#[no_mangle]
pub extern "C" fn publickey_to_compressed(public_key: *mut BSVPublicKey) -> *mut BSVPublicKey {
    let public_key = unsafe { &mut *public_key };
    let public_key = public_key.to_compressed().unwrap();
    Box::into_raw(Box::new(public_key))
}

// pub fn to_decompressed(&self) -> Result<PublicKey, wasm_bindgen::JsError> {
//     Ok(PublicKey(self.0.to_decompressed()?))
// }
#[no_mangle]
pub extern "C" fn publickey_to_decompressed(public_key: *mut BSVPublicKey) -> *mut BSVPublicKey {
    let public_key = unsafe { &mut *public_key };
    let public_key = public_key.to_decompressed().unwrap();
    Box::into_raw(Box::new(public_key))
}

// pub fn encrypt_message(&self, message: &[u8], sender_private_key: &PrivateKey) -> Result<ECIESCiphertext, wasm_bindgen::JsError> {
//     Ok(ECIESCiphertext(self.0.encrypt_message(message, &sender_private_key.0)?))
// }
#[no_mangle]
pub extern "C" fn publickey_encrypt_message(public_key: *mut BSVPublicKey, message: *const u8, message_len: usize, sender_private_key: *mut BSVPrivateKey) -> *mut ECIESCiphertext {
    let public_key = unsafe { &mut *public_key };
    let message = unsafe { std::slice::from_raw_parts(message, message_len) };
    let sender_private_key = unsafe { &mut *sender_private_key };
    let ciphertext = public_key.encrypt_message(message, &sender_private_key).unwrap();
    Box::into_raw(Box::new(ciphertext))
}

// pub fn is_valid_message(&self, message: &[u8], signature: &Signature) -> bool {
//     self.0.verify_message(message, &signature.0).is_ok()
// }
#[no_mangle]
pub extern "C" fn publickey_is_valid_message(public_key: *mut BSVPublicKey, message: *const u8, message_len: usize, signature: *mut Signature) -> bool {
    let public_key = unsafe { &mut *public_key };
    let message = unsafe { std::slice::from_raw_parts(message, message_len) };
    let signature = unsafe { &mut *signature };
    public_key.verify_message(message, &signature).is_ok()
}

// pub fn is_compressed(&self) -> bool {
//     self.0.is_compressed()
// }
#[no_mangle]
pub extern "C" fn publickey_is_compressed(public_key: *mut BSVPublicKey) -> bool {
    let public_key = unsafe { &mut *public_key };
    public_key.is_compressed()
}