use bsv::ECDH as BSVECDH;

// pub fn derive_shared_key(priv_key: &PrivateKey, pub_key: &PublicKey) -> Result<Vec<u8>, wasm_bindgen::JsError> {
//     Ok(BSVECDH::derive_shared_key(&priv_key.0, &pub_key.0)?)
// }
#[no_mangle]
pub extern "C" fn ecdh_derive_shared_key(priv_key: *mut BSVPrivateKey, pub_key: *mut BSVPublicKey) -> *mut u8 {
    let priv_key = unsafe { &mut *priv_key };
    let pub_key = unsafe { &mut *pub_key };
    let shared_key = BSVECDH::derive_shared_key(&priv_key.0, &pub_key.0).unwrap();
    let shared_key = shared_key.as_ptr() as *mut u8;
    std::mem::forget(shared_key);
    shared_key
}