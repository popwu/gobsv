use bsv::ECDSA as BSVECDSA;

// pub fn private_key_from_signature_k(
//     signature: &Signature,
//     public_key: &PublicKey,
//     ephemeral_key: &PrivateKey,
//     preimage: &[u8],
//     hash_algo: SigningHash,
// ) -> Result<PrivateKey, wasm_bindgen::JsError> {
//     Ok(PrivateKey(BSVECDSA::private_key_from_signature_k(
//         &signature.0,
//         &public_key.0,
//         &ephemeral_key.0,
//         preimage,
//         hash_algo.into(),
//     )?))
// }
#[no_mangle]
pub extern "C" fn ecdsa_private_key_from_signature_k(
    signature: *mut BSVSignature,
    public_key: *mut BSVPublicKey,
    ephemeral_key: *mut BSVPrivateKey,
    preimage: *const u8,
    preimage_len: usize,
    hash_algo: SigningHash,
) -> *mut BSVPrivateKey {
    let signature = unsafe { &mut *signature };
    let public_key = unsafe { &mut *public_key };
    let ephemeral_key = unsafe { &mut *ephemeral_key };
    let preimage = unsafe { std::slice::from_raw_parts(preimage, preimage_len) };
    let private_key = BSVECDSA::private_key_from_signature_k(
        &signature,
        &public_key,
        &ephemeral_key,
        preimage,
        hash_algo.into(),
    ).unwrap();
    Box::into_raw(Box::new(private_key))
}

// pub fn sign_with_random_k(private_key: &PrivateKey, preimage: &[u8], hash_algo: SigningHash, reverse_k: bool) -> Result<Signature, wasm_bindgen::JsError> {
//     Ok(Signature(BSVECDSA::sign_with_random_k(&private_key.0, preimage, hash_algo.into(), reverse_k)?))
// }
#[no_mangle]
pub extern "C" fn ecdsa_sign_with_random_k(private_key: *mut BSVPrivateKey, preimage: *const u8, preimage_len: usize, hash_algo: SigningHash, reverse_k: bool) -> *mut BSVSignature {
    let private_key = unsafe { &mut *private_key };
    let preimage = unsafe { std::slice::from_raw_parts(preimage, preimage_len) };
    let signature = BSVECDSA::sign_with_random_k(&private_key, preimage, hash_algo.into(), reverse_k).unwrap();
    Box::into_raw(Box::new(signature))
}

// pub fn sign_with_deterministic_k(private_key: &PrivateKey, preimage: &[u8], hash_algo: SigningHash, reverse_k: bool) -> Result<Signature, wasm_bindgen::JsError> {
//     Ok(Signature(BSVECDSA::sign_with_deterministic_k(&private_key.0, preimage, hash_algo.into(), reverse_k)?))
// }
#[no_mangle]
pub extern "C" fn ecdsa_sign_with_deterministic_k(private_key: *mut BSVPrivateKey, preimage: *const u8, preimage_len: usize, hash_algo: SigningHash, reverse_k: bool) -> *mut BSVSignature {
    let private_key = unsafe { &mut *private_key };
    let preimage = unsafe { std::slice::from_raw_parts(preimage, preimage_len) };
    let signature = BSVECDSA::sign_with_deterministic_k(&private_key, preimage, hash_algo.into(), reverse_k).unwrap();
    Box::into_raw(Box::new(signature))
}

// pub fn sign_digest_with_deterministic_k(private_key: &PrivateKey, digest: &[u8]) -> Result<Signature, wasm_bindgen::JsError> {
//     Ok(Signature(BSVECDSA::sign_digest_with_deterministic_k(&private_key.0, digest)?))
// }
#[no_mangle]
pub extern "C" fn ecdsa_sign_digest_with_deterministic_k(private_key: *mut BSVPrivateKey, digest: *const u8, digest_len: usize) -> *mut BSVSignature {
    let private_key = unsafe { &mut *private_key };
    let digest = unsafe { std::slice::from_raw_parts(digest, digest_len) };
    let signature = BSVECDSA::sign_digest_with_deterministic_k(&private_key, digest).unwrap();
    Box::into_raw(Box::new(signature))
}

// pub fn sign_with_k(private_key: &PrivateKey, ephemeral_key: &PrivateKey, preimage: &[u8], hash_algo: SigningHash) -> Result<Signature, wasm_bindgen::JsError> {
//     Ok(Signature(BSVECDSA::sign_with_k(&private_key.0, &ephemeral_key.0, preimage, hash_algo.into())?))
// }
#[no_mangle]
pub extern "C" fn ecdsa_sign_with_k(private_key: *mut BSVPrivateKey, ephemeral_key: *mut BSVPrivateKey, preimage: *const u8, preimage_len: usize, hash_algo: SigningHash) -> *mut BSVSignature {
    let private_key = unsafe { &mut *private_key };
    let ephemeral_key = unsafe { &mut *ephemeral_key };
    let preimage = unsafe { std::slice::from_raw_parts(preimage, preimage_len) };
    let signature = BSVECDSA::sign_with_k(&private_key, &ephemeral_key, preimage, hash_algo.into()).unwrap();
    Box::into_raw(Box::new(signature))
}

// pub fn verify_digest(message: &[u8], pub_key: &PublicKey, signature: &Signature, hash_algo: SigningHash) -> Result<bool, wasm_bindgen::JsError> {
//     Ok(BSVECDSA::verify_digest(message, &pub_key.0, &signature.0, hash_algo.into())?)
// }
#[no_mangle]
pub extern "C" fn ecdsa_verify_digest(message: *const u8, message_len: usize, pub_key: *mut BSVPublicKey, signature: *mut BSVSignature, hash_algo: SigningHash) -> bool {
    let message = unsafe { std::slice::from_raw_parts(message, message_len) };
    let pub_key = unsafe { &*pub_key };
    let signature = unsafe { &*signature };
    BSVECDSA::verify_digest(message, &pub_key, &signature, hash_algo.into()).unwrap()
}

// pub fn verify_hashbuf(digest: &[u8], pub_key: &PublicKey, signature: &Signature) -> Result<bool, wasm_bindgen::JsError> {
//     Ok(BSVECDSA::verify_hashbuf(digest, &pub_key.0, &signature.0)?)
// }
#[no_mangle]
pub extern "C" fn ecdsa_verify_hashbuf(digest: *const u8, digest_len: usize, pub_key: *mut BSVPublicKey, signature: *mut BSVSignature) -> bool {
    let digest = unsafe { std::slice::from_raw_parts(digest, digest_len) };
    let pub_key = unsafe { &*pub_key };
    let signature = unsafe { &*signature };
    BSVECDSA::verify_hashbuf(digest, &pub_key, &signature).unwrap()
}