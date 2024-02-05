use bsv::CipherKeys as BSVCipherKeys;
use bsv::ECIESCiphertext as BSVECIESCiphertext;
use bsv::ECIES as BSVECIES;

// ECIES

// pub fn encrypt(message: &[u8], sender_priv_key: &PrivateKey, recipient_pub_key: &PublicKey, exclude_pub_key: bool) -> Result<ECIESCiphertext, wasm_bindgen::JsError> {
//     Ok(ECIESCiphertext(BSVECIES::encrypt(message, &sender_priv_key.0, &recipient_pub_key.0, exclude_pub_key)?))
// }
#[no_mangle]
pub extern "C" fn ecies_encrypt(message: *const u8, message_len: usize, sender_priv_key: *mut BSVPrivateKey, recipient_pub_key: *mut BSVPublicKey, exclude_pub_key: bool) -> *mut ECIESCiphertext {
    let message = unsafe { std::slice::from_raw_parts(message, message_len) };
    let sender_priv_key = unsafe { &mut *sender_priv_key };
    let recipient_pub_key = unsafe { &mut *recipient_pub_key };
    let ciphertext = BSVECIES::encrypt(message, &sender_priv_key.0, &recipient_pub_key.0, exclude_pub_key).unwrap();
    Box::into_raw(Box::new(ciphertext))
}

// pub fn encrypt_with_ephemeral_private_key(message: &[u8], recipient_pub_key: &PublicKey) -> Result<ECIESCiphertext, wasm_bindgen::JsError> {
//     Ok(ECIESCiphertext(BSVECIES::encrypt_with_ephemeral_private_key(message, &recipient_pub_key.0)?))
// }
#[no_mangle]
pub extern "C" fn ecies_encrypt_with_ephemeral_private_key(message: *const u8, message_len: usize, recipient_pub_key: *mut BSVPublicKey) -> *mut ECIESCiphertext {
    let message = unsafe { std::slice::from_raw_parts(message, message_len) };
    let recipient_pub_key = unsafe { &mut *recipient_pub_key };
    let ciphertext = BSVECIES::encrypt_with_ephemeral_private_key(message, &recipient_pub_key.0).unwrap();
    Box::into_raw(Box::new(ciphertext))
}

// pub fn decrypt(ciphertext: &ECIESCiphertext, recipient_priv_key: &PrivateKey, sender_pub_key: &PublicKey) -> Result<Vec<u8>, wasm_bindgen::JsError> {
//     Ok(BSVECIES::decrypt(&ciphertext.0, &recipient_priv_key.0, &sender_pub_key.0)?)
// }
#[no_mangle]
pub extern "C" fn ecies_decrypt(ciphertext: *mut ECIESCiphertext, recipient_priv_key: *mut BSVPrivateKey, sender_pub_key: *mut BSVPublicKey) -> *mut u8 {
    let ciphertext = unsafe { &mut *ciphertext };
    let recipient_priv_key = unsafe { &mut *recipient_priv_key };
    let sender_pub_key = unsafe { &mut *sender_pub_key };
    let message = BSVECIES::decrypt(&ciphertext.0, &recipient_priv_key.0, &sender_pub_key.0).unwrap();
    let message = message.as_ptr() as *mut u8;
    std::mem::forget(message);
    message
}

// pub fn derive_cipher_keys(priv_key: &PrivateKey, pub_key: &PublicKey) -> Result<CipherKeys, wasm_bindgen::JsError> {
//     Ok(CipherKeys(BSVECIES::derive_cipher_keys(&priv_key.0, &pub_key.0)?))
// }
#[no_mangle]
pub extern "C" fn ecies_derive_cipher_keys(priv_key: *mut BSVPrivateKey, pub_key: *mut BSVPublicKey) -> *mut ECIESCipherKeys {
    let priv_key = unsafe { &mut *priv_key };
    let pub_key = unsafe { &mut *pub_key };
    let cipher_keys = BSVECIES::derive_cipher_keys(&priv_key.0, &pub_key.0).unwrap();
    Box::into_raw(Box::new(cipher_keys))
}

// CipherKeys

// pub fn get_iv(&self) -> Vec<u8> {
//     self.0.get_iv()
// }
#[no_mangle]
pub extern "C" fn ecies_cipher_keys_get_iv(cipher_keys: *mut ECIESCipherKeys) -> *mut u8 {
    let cipher_keys = unsafe { &mut *cipher_keys };
    let iv = cipher_keys.0.get_iv();
    let iv = iv.as_ptr();
    std::mem::forget(iv);
    iv
}

// pub fn get_ke(&self) -> Vec<u8> {
//     self.0.get_ke()
// }
#[no_mangle]
pub extern "C" fn ecies_cipher_keys_get_ke(cipher_keys: *mut ECIESCipherKeys) -> *mut u8 {
    let cipher_keys = unsafe { &mut *cipher_keys };
    let ke = cipher_keys.0.get_ke();
    let ke = ke.as_ptr();
    std::mem::forget(ke);
    ke
}

// pub fn get_km(&self) -> Vec<u8> {
//     self.0.get_km()
// }
#[no_mangle]
pub extern "C" fn ecies_cipher_keys_get_km(cipher_keys: *mut ECIESCipherKeys) -> *mut u8 {
    let cipher_keys = unsafe { &mut *cipher_keys };
    let km = cipher_keys.0.get_km();
    let km = km.as_ptr();
    std::mem::forget(km);
    km
}

// ECIESCiphertext

// pub fn get_ciphertext(&self) -> Vec<u8> {
//     self.0.get_ciphertext()
// }
#[no_mangle]
pub extern "C" fn ecies_ciphertext_get_ciphertext(ciphertext: *mut ECIESCiphertext) -> *mut u8 {
    let ciphertext = unsafe { &mut *ciphertext };
    let ciphertext = ciphertext.0.get_ciphertext();
    let ciphertext = ciphertext.as_ptr();
    std::mem::forget(ciphertext);
    ciphertext
}

// pub fn get_hmac(&self) -> Vec<u8> {
//     self.0.get_hmac()
// }
#[no_mangle]
pub extern "C" fn ecies_ciphertext_get_hmac(ciphertext: *mut ECIESCiphertext) -> *mut u8 {
    let ciphertext = unsafe { &mut *ciphertext };
    let hmac = ciphertext.0.get_hmac();
    let hmac = hmac.as_ptr();
    std::mem::forget(hmac);
    hmac
}

// pub fn get_cipher_keys(&self) -> Option<CipherKeys> {
//     self.0.get_cipher_keys().map(CipherKeys)
// }
#[no_mangle]
pub extern "C" fn ecies_ciphertext_get_cipher_keys(ciphertext: *mut ECIESCiphertext) -> *mut ECIESCipherKeys {
    let ciphertext = unsafe { &mut *ciphertext };
    let cipher_keys = ciphertext.0.get_cipher_keys().map(ECIESCipherKeys);
    match cipher_keys {
        Some(cipher_keys) => Box::into_raw(Box::new(cipher_keys)),
        None => std::ptr::null_mut(),
    }
}

// pub fn to_bytes(&self) -> Vec<u8> {
//     self.0.to_bytes()
// }
#[no_mangle]
pub extern "C" fn ecies_ciphertext_to_bytes(ciphertext: *mut ECIESCiphertext) -> *mut u8 {
    let ciphertext = unsafe { &mut *ciphertext };
    let bytes = ciphertext.0.to_bytes();
    let bytes = bytes.as_ptr();
    std::mem::forget(bytes);
    bytes
}

// pub fn extract_public_key(&self) -> Result<PublicKey, wasm_bindgen::JsError> {
//     Ok(PublicKey(self.0.extract_public_key()?))
// }
#[no_mangle]
pub extern "C" fn ecies_ciphertext_extract_public_key(ciphertext: *mut ECIESCiphertext) -> *mut BSVPublicKey {
    let ciphertext = unsafe { &mut *ciphertext };
    let pub_key = ciphertext.0.extract_public_key().unwrap();
    Box::into_raw(Box::new(pub_key))
}

// pub fn from_bytes(buffer: &[u8], has_pub_key: bool) -> Result<ECIESCiphertext, wasm_bindgen::JsError> {
//     Ok(ECIESCiphertext(BSVECIESCiphertext::from_bytes(buffer, has_pub_key)?))
// }
#[no_mangle]
pub extern "C" fn ecies_ciphertext_from_bytes(buffer: *const u8, buffer_len: usize, has_pub_key: bool) -> *mut ECIESCiphertext {
    let buffer = unsafe { std::slice::from_raw_parts(buffer, buffer_len) };
    let ciphertext = BSVECIESCiphertext::from_bytes(buffer, has_pub_key).unwrap();
    Box::into_raw(Box::new(ciphertext))
}