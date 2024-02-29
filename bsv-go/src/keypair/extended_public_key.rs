use bsv::ExtendedPrivateKey as BSVExtendedPrivateKey;
use bsv::ExtendedPublicKey as BSVExtendedPublicKey;
use bsv::PublicKey as BSVPublicKey;

use std::ffi::CStr;
use std::ffi::CString;

#[repr(C)]
pub struct ByteArray {
    data: *mut u8,
    len: usize,
}

// pub fn get_public_key(&self) -> PublicKey {
//     PublicKey(self.0.get_public_key())
// }
#[no_mangle]
pub extern "C" fn extended_public_key_get_public_key(
    extended_public_key: *mut BSVExtendedPublicKey,
) -> *mut BSVPublicKey {
    let extended_public_key = unsafe { &mut *extended_public_key };
    let public_key = extended_public_key.get_public_key();
    Box::into_raw(Box::new(public_key))
}

// pub fn from_xpriv(xpriv: &ExtendedPrivateKey) -> Self {
//     ExtendedPublicKey(BSVExtendedPublicKey::from_xpriv(&xpriv.0))
// }
#[no_mangle]
pub extern "C" fn extended_public_key_from_xpriv(
    xpriv: *mut BSVExtendedPrivateKey,
) -> *mut BSVExtendedPublicKey {
    let xpriv = unsafe { &mut *xpriv };
    let extended_public_key = BSVExtendedPublicKey::from_xpriv(&xpriv);
    Box::into_raw(Box::new(extended_public_key))
}

// pub fn get_chain_code(&self) -> Vec<u8> {
//     self.0.get_chain_code()
// }
#[no_mangle]
pub extern "C" fn extended_public_key_get_chain_code(
    extended_public_key: *mut BSVExtendedPublicKey,
) -> *mut ByteArray {
    let extended_public_key = unsafe { &mut *extended_public_key };
    let mut chain_code = extended_public_key.get_chain_code();
    let len = chain_code.len();
    let data = chain_code.as_mut_ptr();
    std::mem::forget(chain_code);
    Box::into_raw(Box::new(ByteArray { data, len }))
}

// // #[cfg_attr(all(feature = "wasm-bindgen-keypair"), wasm_bindgen(js_name = getDepth))]
// pub fn get_depth(&self) -> u8 {
//     self.0.get_depth()
// }
#[no_mangle]
pub extern "C" fn extended_public_key_get_depth(
    extended_public_key: *mut BSVExtendedPublicKey,
) -> u8 {
    let extended_public_key = unsafe { &mut *extended_public_key };
    extended_public_key.get_depth()
}

// pub fn get_parent_fingerprint(&self) -> Vec<u8> {
//     self.0.get_parent_fingerprint()
// }
#[no_mangle]
pub extern "C" fn extended_public_key_get_parent_fingerprint(
    extended_public_key: *mut BSVExtendedPublicKey,
) -> *mut ByteArray {
    let extended_public_key = unsafe { &mut *extended_public_key };
    let mut parent_fingerprint = extended_public_key.get_parent_fingerprint();
    let len = parent_fingerprint.len();
    let data = parent_fingerprint.as_mut_ptr();
    std::mem::forget(parent_fingerprint);
    Box::into_raw(Box::new(ByteArray { data, len }))
}

// pub fn get_index(&self) -> u32 {
//     self.0.get_index()
// }
#[no_mangle]
pub extern "C" fn extended_public_key_get_index(
    extended_public_key: *mut BSVExtendedPublicKey,
) -> u32 {
    let extended_public_key = unsafe { &mut *extended_public_key };
    extended_public_key.get_index()
}

// pub fn derive(&self, index: u32) -> Result<ExtendedPublicKey, wasm_bindgen::JsError> {
//     Ok(Self(self.0.derive(index)?))
// }
#[no_mangle]
pub extern "C" fn extended_public_key_derive(
    extended_public_key: *mut BSVExtendedPublicKey,
    index: u32,
) -> *mut BSVExtendedPublicKey {
    let extended_public_key = unsafe { &mut *extended_public_key };
    let extended_public_key = extended_public_key.derive(index).unwrap();
    Box::into_raw(Box::new(extended_public_key))
}

// pub fn derive_from_path(&self, path: &str) -> Result<ExtendedPublicKey, wasm_bindgen::JsError> {
//     Ok(Self(self.0.derive_from_path(path)?))
// }
#[no_mangle]
pub extern "C" fn extended_public_key_derive_from_path(
    extended_public_key: *mut BSVExtendedPublicKey,
    path: *mut libc::c_char,
) -> *mut BSVExtendedPublicKey {
    let extended_public_key = unsafe { &mut *extended_public_key };
    let path = unsafe { CStr::from_ptr(path) };
    let path = path.to_str().unwrap();
    let extended_public_key = extended_public_key.derive_from_path(path).unwrap();
    Box::into_raw(Box::new(extended_public_key))
}

// pub fn from_seed(seed: &[u8]) -> Result<ExtendedPublicKey, wasm_bindgen::JsError> {
//     Ok(ExtendedPublicKey(BSVExtendedPublicKey::from_seed(seed)?))
// }
#[no_mangle]
pub extern "C" fn extended_public_key_from_seed(
    seed: *mut u8,
    len: usize,
) -> *mut BSVExtendedPublicKey {
    let seed = unsafe { std::slice::from_raw_parts(seed, len) };
    let extended_public_key = BSVExtendedPublicKey::from_seed(seed).unwrap();
    Box::into_raw(Box::new(extended_public_key))
}

// pub fn from_random() -> Result<ExtendedPublicKey, wasm_bindgen::JsError> {
//     Ok(ExtendedPublicKey(BSVExtendedPublicKey::from_random()?))
// }
#[no_mangle]
pub extern "C" fn extended_public_key_from_random() -> *mut BSVExtendedPublicKey {
    let extended_public_key = BSVExtendedPublicKey::from_random().unwrap();
    Box::into_raw(Box::new(extended_public_key))
}

// pub fn from_string(xpub_string: &str) -> Result<ExtendedPublicKey, wasm_bindgen::JsError> {
//     Ok(ExtendedPublicKey(BSVExtendedPublicKey::from_string(xpub_string)?))
// }
#[no_mangle]
pub extern "C" fn extended_public_key_from_string(
    xpub_string: *mut libc::c_char,
) -> *mut BSVExtendedPublicKey {
    let xpub_string = unsafe { CStr::from_ptr(xpub_string) };
    let xpub_string = xpub_string.to_str().unwrap();
    let extended_public_key = BSVExtendedPublicKey::from_string(xpub_string).unwrap();
    Box::into_raw(Box::new(extended_public_key))
}

// pub fn to_string(&self) -> Result<String, wasm_bindgen::JsError> {
//     Ok(self.0.to_string()?)
// }
#[no_mangle]
pub extern "C" fn extended_public_key_to_string(
    extended_public_key: *mut BSVExtendedPublicKey,
) -> *mut libc::c_char {
    let extended_public_key = unsafe { &mut *extended_public_key };
    let string = extended_public_key.to_string().unwrap();
    let c_str = CString::new(string).unwrap();
    c_str.into_raw()
}
