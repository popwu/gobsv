// use crate::{ecdsa::SigningHash, keypair::public_key::PublicKey};
use bsv::PublicKey as BSVPublicKey;
use bsv::SigningHash;
use bsv::{RecoveryInfo as BSVRecoveryInfo, Signature as BSVSignature};
use std::ffi::CString;

#[repr(C)]
pub struct ByteArray {
    data: *mut u8,
    len: usize,
}
// pub fn from_der(bytes: &[u8]) -> Result<Signature, wasm_bindgen::JsError> {
//     Ok(Signature(BSVSignature::from_der(bytes)?))
// }
#[no_mangle]
pub extern "C" fn signature_from_der(bytes: &[u8]) -> *mut BSVSignature {
    let signature = BSVSignature::from_der(bytes).unwrap();
    Box::into_raw(Box::new(signature))
}

// pub fn from_hex_der(hex: &str) -> Result<Signature, wasm_bindgen::JsError> {
//     Ok(Signature(BSVSignature::from_hex_der(hex)?))
// }
#[no_mangle]
pub extern "C" fn signature_from_hex_der(hex: &str) -> *mut BSVSignature {
    let signature = BSVSignature::from_hex_der(hex).unwrap();
    Box::into_raw(Box::new(signature))
}

// pub fn from_compact_bytes(compact_bytes: &[u8]) -> Result<Signature, wasm_bindgen::JsError> {
//     Ok(Signature(BSVSignature::from_compact_impl(compact_bytes)?))
// }
#[no_mangle]
pub extern "C" fn signature_from_compact_bytes(compact_bytes: &[u8]) -> *mut BSVSignature {
    let signature = BSVSignature::from_compact_impl(compact_bytes).unwrap();
    Box::into_raw(Box::new(signature))
}

// pub fn recover_public_key(&self, message: &[u8], hash_algo: SigningHash) -> Result<PublicKey, wasm_bindgen::JsError> {
//     Ok(PublicKey(self.0.recover_public_key(message, hash_algo.into())?))
// }
#[no_mangle]
pub extern "C" fn signature_recover_public_key(
    signature: *mut BSVSignature,
    message: *const u8,
    message_len: usize,
    hash_algo: SigningHash,
) -> *mut BSVPublicKey {
    let signature = unsafe { &mut *signature };
    let message = unsafe { std::slice::from_raw_parts(message, message_len) };
    let public_key = signature
        .recover_public_key(message, hash_algo.into())
        .unwrap();
    Box::into_raw(Box::new(public_key))
}

// pub fn recover_public_key_from_digest(&self, digest: &[u8]) -> Result<PublicKey, wasm_bindgen::JsError> {
//     Ok(PublicKey(self.0.recover_public_key_from_digest(digest)?))
// }
#[no_mangle]
pub extern "C" fn signature_recover_public_key_from_digest(
    signature: *mut BSVSignature,
    digest: *const u8,
    digest_len: usize,
) -> *mut BSVPublicKey {
    let signature = unsafe { &mut *signature };
    let digest = unsafe { std::slice::from_raw_parts(digest, digest_len) };
    let public_key = signature.recover_public_key_from_digest(digest).unwrap();
    Box::into_raw(Box::new(public_key))
}

// pub fn to_der_hex(&self) -> String {
//     BSVSignature::to_der_hex(&self.0)
// }
#[no_mangle]
pub extern "C" fn signature_to_der_hex(signature: *mut BSVSignature) -> *mut libc::c_char {
    let signature = unsafe { &mut *signature };
    let hex = BSVSignature::to_der_hex(&signature);
    let c_str = CString::new(hex).unwrap();
    c_str.into_raw()
}

// pub fn to_der_bytes(&self) -> Vec<u8> {
//     BSVSignature::to_der_bytes(&self.0)
// }
#[no_mangle]
pub extern "C" fn signature_to_der_bytes(signature: *mut BSVSignature) -> ByteArray {
    let signature = unsafe { &mut *signature };
    let bytes = BSVSignature::to_der_bytes(&signature);
    let len = bytes.len();
    let out = bytes.into_boxed_slice();
    ByteArray {
        data: Box::into_raw(out) as *mut u8,
        len,
    }
}

// pub fn to_compact_bytes(&self, recovery_info: Option<RecoveryInfo>) -> Vec<u8> {
//     let r = match recovery_info {
//         Some(v) => Some(v.0),
//         None => None,
//     };
//     BSVSignature::to_compact_bytes(&self.0, r)
// }
#[no_mangle]
pub extern "C" fn signature_to_compact_bytes(
    signature: *mut BSVSignature,
    recovery_info: Option<BSVRecoveryInfo>,
) -> ByteArray {
    let signature = unsafe { &mut *signature };
    let r = match recovery_info {
        Some(v) => Some(v),
        None => None,
    };
    let bytes = BSVSignature::to_compact_bytes(&signature, r);
    let len = bytes.len();
    let out = bytes.into_boxed_slice();
    ByteArray {
        data: Box::into_raw(out) as *mut u8,
        len,
    }
}

// pub fn r(&self) -> Vec<u8> {
//     BSVSignature::r(&self.0)
// }
#[no_mangle]
pub extern "C" fn signature_r(signature: *mut BSVSignature) -> ByteArray {
    let signature = unsafe { &mut *signature };
    let bytes = BSVSignature::r(&signature);
    let len = bytes.len();
    let out = bytes.into_boxed_slice();
    ByteArray {
        data: Box::into_raw(out) as *mut u8,
        len,
    }
}

// pub fn r_hex(&self) -> String {
//     BSVSignature::r_hex(&self.0)
// }
#[no_mangle]
pub extern "C" fn signature_r_hex(signature: *mut BSVSignature) -> *mut libc::c_char {
    let signature = unsafe { &mut *signature };
    let hex = BSVSignature::r_hex(&signature);
    let c_str = CString::new(hex).unwrap();
    c_str.into_raw()
}

// pub fn s(&self) -> Vec<u8> {
//     BSVSignature::s(&self.0)
// }
#[no_mangle]
pub extern "C" fn signature_s(signature: *mut BSVSignature) -> ByteArray {
    let signature = unsafe { &mut *signature };
    let bytes = BSVSignature::s(&signature);
    let len = bytes.len();
    let out = bytes.into_boxed_slice();
    ByteArray {
        data: Box::into_raw(out) as *mut u8,
        len,
    }
}

// pub fn s_hex(&self) -> String {
//     BSVSignature::s_hex(&self.0)
// }
#[no_mangle]
pub extern "C" fn signature_s_hex(signature: *mut BSVSignature) -> *mut libc::c_char {
    let signature = unsafe { &mut *signature };
    let hex = BSVSignature::s_hex(&signature);
    let c_str = CString::new(hex).unwrap();
    c_str.into_raw()
}

// pub fn to_compact_hex(&self, recovery_info: Option<RecoveryInfo>) -> String {
//     let r = match recovery_info {
//         Some(v) => Some(v.0),
//         None => None,
//     };
//     BSVSignature::to_compact_hex(&self.0, r)
// }
#[no_mangle]
pub extern "C" fn signature_to_compact_hex(
    signature: *mut BSVSignature,
    recovery_info: Option<BSVRecoveryInfo>,
) -> *mut libc::c_char {
    let signature = unsafe { &mut *signature };
    let r = match recovery_info {
        Some(v) => Some(v),
        None => None,
    };
    let hex = BSVSignature::to_compact_hex(&signature, r);
    let c_str = CString::new(hex).unwrap();
    c_str.into_raw()
}

// pub fn verify_message(&self, message: &[u8], pub_key: &PublicKey) -> bool {
//     BSVSignature::verify_message(&self.0, message, &pub_key.0)
// }
#[no_mangle]
pub extern "C" fn signature_verify_message(
    signature: *mut BSVSignature,
    message: *const u8,
    message_len: usize,
    pub_key: *mut BSVPublicKey,
) -> bool {
    let signature = unsafe { &mut *signature };
    let message = unsafe { std::slice::from_raw_parts(message, message_len) };
    let pub_key = unsafe { &mut *pub_key };
    BSVSignature::verify_message(&signature, message, &pub_key)
}
