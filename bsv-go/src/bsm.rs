use bsv::BSM as BSVBSM;
use bsv::P2PKHAddress as BSVP2PKHAddress;
use bsv::Signature as BSVSignature

// pub fn is_valid_message(message: &[u8], signature: &Signature, address: &P2PKHAddress) -> bool {
//     BSVBSM::verify_message(message, &signature.0, &address.0).is_ok()
// }
#[no_mangle]
pub extern "C" fn bsm_is_valid_message(message: *const u8, message_len: usize, signature: *mut BSVSignature, address: *mut BSVP2PKHAddress) -> bool {
    let message = unsafe { std::slice::from_raw_parts(message, message_len) };
    let signature = unsafe { &mut *signature };
    let address = unsafe { &mut *address };
    BSVBSM::verify_message(message, &signature.0, &address.0).is_ok()
}

// pub fn verify_message(message: &[u8], signature: &Signature, address: &P2PKHAddress) -> Result<bool, wasm_bindgen::JsError> {
//     Ok(BSVBSM::verify_message(message, &signature.0, &address.0)?)
// }
#[no_mangle]
pub extern "C" fn bsm_verify_message(message: *const u8, message_len: usize, signature: *mut BSVSignature, address: *mut BSVP2PKHAddress) -> bool {
    let message = unsafe { std::slice::from_raw_parts(message, message_len) };
    let signature = unsafe { &mut *signature };
    let address = unsafe { &mut *address };
    BSVBSM::verify_message(message, &signature.0, &address.0).unwrap()
}


// pub fn sign_message(priv_key: &PrivateKey, message: &[u8]) -> Result<Signature, wasm_bindgen::JsError> {
//     Ok(Signature(BSVBSM::sign_message(&priv_key.0, message)?))
// }
#[no_mangle]
pub extern "C" fn bsm_sign_message(priv_key: *mut BSVPrivateKey, message: *const u8, message_len: usize) -> *mut BSVSignature {
    let priv_key = unsafe { &mut *priv_key };
    let message = unsafe { std::slice::from_raw_parts(message, message_len) };
    let signature = BSVBSM::sign_message(&priv_key.0, message).unwrap();
    Box::into_raw(Box::new(signature))
}

// pub fn sign_message_with_k(priv_key: &PrivateKey, ephemeral_key: &PrivateKey, message: &[u8]) -> Result<Signature, wasm_bindgen::JsError> {
//     Ok(Signature(BSVBSM::sign_message_with_k(&priv_key.0, &ephemeral_key.0, message)?))
// }
#[no_mangle]
pub extern "C" fn bsm_sign_message_with_k(priv_key: *mut BSVPrivateKey, ephemeral_key: *mut BSVPrivateKey, message: *const u8, message_len: usize) -> *mut BSVSignature {
    let priv_key = unsafe { &mut *priv_key };
    let ephemeral_key = unsafe { &mut *ephemeral_key };
    let message = unsafe { std::slice::from_raw_parts(message, message_len) };
    let signature = BSVBSM::sign_message_with_k(&priv_key.0, &ephemeral_key.0, message).unwrap();
    Box::into_raw(Box::new(signature))
}
