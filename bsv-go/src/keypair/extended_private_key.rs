use bsv::ExtendedPrivateKey as BSVExtendedPrivateKey;
use bsv::PrivateKey as BSVPrivateKey;
use bsv::PublicKey as BSVPublicKey;

use std::ffi::CStr;
use std::ffi::CString;

#[repr(C)]
pub struct ByteArray {
    data: *mut u8,
    len: usize,
}

impl ByteArray {
    pub fn to_vec(&self) -> Vec<u8> {
        unsafe {
            let slice = std::slice::from_raw_parts(self.data, self.len);
            slice.to_vec()
        }
    }
}

// pub fn get_private_key(&self) -> PrivateKey {
//     PrivateKey(self.0.get_private_key())
// }
#[no_mangle]
pub extern "C" fn extended_private_key_get_private_key(
    extended_private_key: *mut BSVExtendedPrivateKey,
) -> *mut BSVPrivateKey {
    let extended_private_key = unsafe { &mut *extended_private_key };
    let private_key = extended_private_key.get_private_key();
    Box::into_raw(Box::new(private_key))
}

// // #[cfg_attr(all(feature = "wasm-bindgen-keypair"), wasm_bindgen(js_name = getPublicKey))]
// pub fn get_public_key(&self) -> PublicKey {
//     PublicKey(self.0.get_public_key())
// }
#[no_mangle]
pub extern "C" fn extended_private_key_get_public_key(
    extended_private_key: *mut BSVExtendedPrivateKey,
) -> *mut BSVPublicKey {
    let extended_private_key = unsafe { &mut *extended_private_key };
    let public_key = extended_private_key.get_public_key();
    Box::into_raw(Box::new(public_key))
}

// // #[cfg_attr(all(feature = "wasm-bindgen-keypair"), wasm_bindgen(js_name = getChainCode))]
// pub fn get_chain_code(&self) -> Vec<u8> {
//     self.0.get_chain_code()
// }
#[no_mangle]
pub extern "C" fn extended_private_key_get_chain_code(
    extended_private_key: *mut BSVExtendedPrivateKey,
) -> *mut ByteArray {
    let extended_private_key = unsafe { &mut *extended_private_key };
    let mut chain_code = extended_private_key.get_chain_code();
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
pub extern "C" fn extended_private_key_get_depth(
    extended_private_key: *mut BSVExtendedPrivateKey,
) -> u8 {
    let extended_private_key = unsafe { &mut *extended_private_key };
    extended_private_key.get_depth()
}

// // #[cfg_attr(all(feature = "wasm-bindgen-keypair"), wasm_bindgen(js_name = getParentFingerprint))]
// pub fn get_parent_fingerprint(&self) -> Vec<u8> {
//     self.0.get_parent_fingerprint()
// }
#[no_mangle]
pub extern "C" fn extended_private_key_get_parent_fingerprint(
    extended_private_key: *mut BSVExtendedPrivateKey,
) -> *mut ByteArray {
    let extended_private_key = unsafe { &mut *extended_private_key };
    let mut parent_fingerprint = extended_private_key.get_parent_fingerprint();
    let len = parent_fingerprint.len();
    let data = parent_fingerprint.as_mut_ptr();
    std::mem::forget(parent_fingerprint);
    Box::into_raw(Box::new(ByteArray { data, len }))
}

// // #[cfg_attr(all(feature = "wasm-bindgen-keypair"), wasm_bindgen(js_name = getIndex))]
// pub fn get_index(&self) -> u32 {
//     self.0.get_index()
// }
#[no_mangle]
pub extern "C" fn extended_private_key_get_index(
    extended_private_key: *mut BSVExtendedPrivateKey,
) -> u32 {
    let extended_private_key = unsafe { &mut *extended_private_key };
    extended_private_key.get_index()
}

// pub fn derive(&self, index: u32) -> Result<ExtendedPrivateKey, wasm_bindgen::JsError> {
//     Ok(ExtendedPrivateKey(self.0.derive(index)?))
// }
#[no_mangle]
pub extern "C" fn extended_private_key_derive(
    extended_private_key: *mut BSVExtendedPrivateKey,
    index: u32,
) -> *mut BSVExtendedPrivateKey {
    let extended_private_key = unsafe { &mut *extended_private_key };
    let extended_private_key = extended_private_key.derive(index).unwrap();
    Box::into_raw(Box::new(extended_private_key))
}

// pub fn derive_from_path(&self, path: &str) -> Result<ExtendedPrivateKey, wasm_bindgen::JsError> {
//     Ok(ExtendedPrivateKey(self.0.derive_from_path(path)?))
// }
#[no_mangle]
pub extern "C" fn extended_private_key_derive_from_path(
    extended_private_key: *mut BSVExtendedPrivateKey,
    path: *mut libc::c_char,
) -> *mut BSVExtendedPrivateKey {
    let extended_private_key = unsafe { &mut *extended_private_key };
    let path = unsafe { CStr::from_ptr(path) };
    let path = path.to_str().unwrap();
    let extended_private_key = extended_private_key.derive_from_path(path).unwrap();
    Box::into_raw(Box::new(extended_private_key))
}

// pub fn from_seed(seed: &[u8]) -> Result<ExtendedPrivateKey, wasm_bindgen::JsError> {
//     Ok(ExtendedPrivateKey(BSVExtendedPrivateKey::from_seed_impl(seed)?))
// }
#[no_mangle]
pub extern "C" fn extended_private_key_from_seed(
    seed: *mut u8,
    len: usize,
) -> *mut BSVExtendedPrivateKey {
    let seed = unsafe { std::slice::from_raw_parts(seed, len) };
    let extended_private_key = BSVExtendedPrivateKey::from_seed_impl(seed).unwrap();
    Box::into_raw(Box::new(extended_private_key))
}

// pub fn from_random() -> Result<ExtendedPrivateKey, wasm_bindgen::JsError> {
//     Ok(ExtendedPrivateKey(BSVExtendedPrivateKey::from_random()?))
// }
#[no_mangle]
pub extern "C" fn extended_private_key_from_random() -> *mut BSVExtendedPrivateKey {
    let extended_private_key = BSVExtendedPrivateKey::from_random().unwrap();
    Box::into_raw(Box::new(extended_private_key))
}

// pub fn from_string(xprv_string: &str) -> Result<ExtendedPrivateKey, wasm_bindgen::JsError> {
//     Ok(ExtendedPrivateKey(BSVExtendedPrivateKey::from_string(xprv_string)?))
// }
#[no_mangle]
pub extern "C" fn extended_private_key_from_string(
    xprv_string: *mut libc::c_char,
) -> *mut BSVExtendedPrivateKey {
    let xprv_string = unsafe { CStr::from_ptr(xprv_string) };
    let xprv_string = xprv_string.to_str().unwrap();
    let extended_private_key = BSVExtendedPrivateKey::from_string(xprv_string).unwrap();
    Box::into_raw(Box::new(extended_private_key))
}

// pub fn to_string(&self) -> Result<String, wasm_bindgen::JsError> {
//     Ok(self.0.to_string()?)
// }
#[no_mangle]
pub extern "C" fn extended_private_key_to_string(
    extended_private_key: *mut BSVExtendedPrivateKey,
) -> *mut libc::c_char {
    let extended_private_key = unsafe { &mut *extended_private_key };
    let string = extended_private_key.to_string().unwrap();
    let c_str = CString::new(string).unwrap();
    c_str.into_raw()
}

// pub fn from_mnemonic(mnemonic: &[u8], passphrase: Option<Vec<u8>>) -> Result<ExtendedPrivateKey, wasm_bindgen::JsError> {
//     Ok(ExtendedPrivateKey(BSVExtendedPrivateKey::from_mnemonic(mnemonic, passphrase)?))
// }
#[no_mangle]
pub extern "C" fn extended_private_key_from_mnemonic(
    mnemonic: *mut u8,
    mnemonic_len: usize,
    passphrase: *mut ByteArray,
) -> *mut BSVExtendedPrivateKey {
    let mnemonic = unsafe { std::slice::from_raw_parts(mnemonic, mnemonic_len) };
    let passphrase = if passphrase.is_null() {
        None
    } else {
        let passphrase = unsafe { &*passphrase };
        let passphrase = passphrase.to_vec();
        Some(passphrase)
    };
    let extended_private_key = BSVExtendedPrivateKey::from_mnemonic(mnemonic, passphrase).unwrap();
    Box::into_raw(Box::new(extended_private_key))
}
