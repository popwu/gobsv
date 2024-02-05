use bsv::P2PKHAddress as BSVP2PKHAddress;

#[repr(C)]
pub struct ByteArray {
    data: *mut u8,
    len: usize,
}

// #[no_mangle]
// pub extern "C" fn privatekey_to_bytes(private_key: *mut BSVPrivateKey) -> ByteArray {
//     let private_key = unsafe { &mut *private_key };
//     let bytes = private_key.to_bytes();
//     let len = bytes.len();
//     let out = bytes.into_boxed_slice();
//     ByteArray { data: Box::into_raw(out) as *mut u8, len }
// }

// pub fn from_pubkey_hash(hash_bytes: &[u8]) -> Result<P2PKHAddress, wasm_bindgen::JsError> {
//         Ok(P2PKHAddress(BSVP2PKHAddress::from_pubkey_hash(hash_bytes)?))
//     }
#[no_mangle]
pub extern "C" fn p2pkhaddress_from_pubkey_hash(hash_bytes: *mut u8, len: usize) -> *mut BSVP2PKHAddress {
    let hash_bytes = unsafe { std::slice::from_raw_parts(hash_bytes, len) };
    let address = BSVP2PKHAddress::from_pubkey_hash(hash_bytes).unwrap();
    Box::into_raw(Box::new(address))
}


// pub fn from_pubkey(pub_key: &PublicKey) -> Result<P2PKHAddress, wasm_bindgen::JsError> {
//     Ok(P2PKHAddress(BSVP2PKHAddress::from_pubkey(&pub_key.0)?))
// }
#[no_mangle]
pub extern "C" fn p2pkhaddress_from_pubkey(pub_key: *mut BSVPublicKey) -> *mut BSVP2PKHAddress {
    let pub_key = unsafe { &mut *pub_key };
    let address = BSVP2PKHAddress::from_pubkey(&pub_key).unwrap();
    Box::into_raw(Box::new(address))
}

// pub fn set_chain_params(&self, chain_params: &ChainParams) -> Result<P2PKHAddress, wasm_bindgen::JsError> {
//     Ok(P2PKHAddress(BSVP2PKHAddress::set_chain_params(&self.0, &chain_params.0)?))
// }
#[no_mangle]
pub extern "C" fn p2pkhaddress_set_chain_params(address: *mut BSVP2PKHAddress, chain_params: *mut BSVChainParams) -> *mut BSVP2PKHAddress {
    let address = unsafe { &mut *address };
    let chain_params = unsafe { &mut *chain_params };
    let address = BSVP2PKHAddress::set_chain_params(&address, &chain_params).unwrap();
    Box::into_raw(Box::new(address))
}

// pub fn to_string(&self) -> Result<String, wasm_bindgen::JsError> {
//     Ok(BSVP2PKHAddress::to_string(&self.0)?)
// }
#[no_mangle]
pub extern "C" fn p2pkhaddress_to_string(address: *mut BSVP2PKHAddress) -> *mut libc::c_char {
    let address = unsafe { &mut *address };
    let string = BSVP2PKHAddress::to_string(&address).unwrap();
    let c_str = CString::new(string).unwrap();
    c_str.into_raw()
}

// pub fn from_string(address_string: &str) -> Result<P2PKHAddress, wasm_bindgen::JsError> {
//     Ok(P2PKHAddress(BSVP2PKHAddress::from_string(address_string)?))
// }
#[no_mangle]
pub extern "C" fn p2pkhaddress_from_string(address_string: *mut libc::c_char) -> *mut BSVP2PKHAddress {
    let address_string = unsafe { CStr::from_ptr(address_string) };
    let address_string = address_string.to_str().unwrap();
    let address = BSVP2PKHAddress::from_string(address_string).unwrap();
    Box::into_raw(Box::new(address))
}

// pub fn get_locking_script(&self) -> Result<Script, wasm_bindgen::JsError> {
//     Ok(Script(BSVP2PKHAddress::get_locking_script(&self.0)?))
// }
#[no_mangle]
pub extern "C" fn p2pkhaddress_get_locking_script(address: *mut BSVP2PKHAddress) -> *mut BSVScript {
    let address = unsafe { &mut *address };
    let script = BSVP2PKHAddress::get_locking_script(&address).unwrap();
    Box::into_raw(Box::new(script))
}

// pub fn get_unlocking_script(&self, pub_key: &PublicKey, sig: &SighashSignature) -> Result<Script, wasm_bindgen::JsError> {
//     Ok(Script(BSVP2PKHAddress::get_unlocking_script(&self.0, &pub_key.0, &sig.0)?))
// }
#[no_mangle]
pub extern "C" fn p2pkhaddress_get_unlocking_script(address: *mut BSVP2PKHAddress, pub_key: *mut BSVPublicKey, sig: *mut BSVSighashSignature) -> *mut BSVScript {
    let address = unsafe { &mut *address };
    let pub_key = unsafe { &mut *pub_key };
    let sig = unsafe { &mut *sig };
    let script = BSVP2PKHAddress::get_unlocking_script(&address, &pub_key, &sig).unwrap();
    Box::into_raw(Box::new(script))
}

// pub fn verify_bitcoin_message(&self, message: &[u8], signature: &Signature) -> Result<bool, wasm_bindgen::JsError> {
//     Ok(BSVP2PKHAddress::verify_bitcoin_message(&self.0, message, &signature.0)?)
// }
#[no_mangle]
pub extern "C" fn p2pkhaddress_verify_bitcoin_message(address: *mut BSVP2PKHAddress, message: *mut u8, message_len: usize, signature: *mut BSVSignature) -> bool {
    let address = unsafe { &mut *address };
    let message = unsafe { std::slice::from_raw_parts(message, message_len) };
    let signature = unsafe { &mut *signature };
    BSVP2PKHAddress::verify_bitcoin_message(&address, message, &signature).unwrap()
}