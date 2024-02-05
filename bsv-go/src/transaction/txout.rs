use bsv::TxOut as BSVTxOut;

// #[wasm_bindgen(constructor)]
// pub fn new(value: u64, script_pub_key: &Script) -> TxOut {
//     TxOut(BSVTxOut::new(value, &script_pub_key.0))
// }
#[no_mangle]
pub extern "C" fn txout_new(value: u64, script_pub_key: *mut BSVScript) -> *mut BSVTxOut {
    let script_pub_key = unsafe { &mut *script_pub_key };
    let tx_out = BSVTxOut::new(value, &script_pub_key.0);
    Box::into_raw(Box::new(tx_out))
}

// pub fn get_satoshis(&self) -> u64 {
//     self.0.get_satoshis()
// }
#[no_mangle]
pub extern "C" fn txout_get_satoshis(tx_out: *mut BSVTxOut) -> u64 {
    let tx_out = unsafe { &mut *tx_out };
    tx_out.get_satoshis()
}

// pub fn get_satoshis_as_bytes(&self) -> Vec<u8> {
//     self.0.get_satoshis_as_bytes()
// }
#[no_mangle]
pub extern "C" fn txout_get_satoshis_as_bytes(tx_out: *mut BSVTxOut) -> ByteArray {
    let tx_out = unsafe { &mut *tx_out };
    let bytes = tx_out.get_satoshis_as_bytes();
    let len = bytes.len();
    let out = bytes.into_boxed_slice();
    ByteArray { data: Box::into_raw(out) as *mut u8, len }
}

// pub fn get_script_pub_key_size(&self) -> usize {
//     self.0.get_script_pub_key_size()
// }
#[no_mangle]
pub extern "C" fn txout_get_script_pub_key_size(tx_out: *mut BSVTxOut) -> usize {
    let tx_out = unsafe { &mut *tx_out };
    tx_out.get_script_pub_key_size()
}

// pub fn get_script_pub_key(&self) -> Script {
//     Script(self.0.get_script_pub_key())
// }
#[no_mangle]
pub extern "C" fn txout_get_script_pub_key(tx_out: *mut BSVTxOut) -> *mut BSVScript {
    let tx_out = unsafe { &mut *tx_out };
    let script = tx_out.get_script_pub_key();
    Box::into_raw(Box::new(script))
}

// pub fn get_script_pub_key_hex(&self) -> String {
//     self.0.get_script_pub_key_hex()
// }
#[no_mangle]
pub extern "C" fn txout_get_script_pub_key_hex(tx_out: *mut BSVTxOut) -> *mut libc::c_char {
    let tx_out = unsafe { &mut *tx_out };
    let string = tx_out.get_script_pub_key_hex();
    let c_str = CString::new(string).unwrap();
    c_str.into_raw()
}

// pub fn from_hex(hex_str: &str) -> Result<TxOut, wasm_bindgen::JsError> {
//     Ok(TxOut(BSVTxOut::from_hex(hex_str)?))
// }
#[no_mangle]
pub extern "C" fn txout_from_hex(hex_str: *mut libc::c_char) -> *mut BSVTxOut {
    let hex_str = unsafe { CStr::from_ptr(hex_str) };
    let hex_str = hex_str.to_str().unwrap();
    let tx_out = BSVTxOut::from_hex(hex_str).unwrap();
    Box::into_raw(Box::new(tx_out))
}

// pub fn to_bytes(&self) -> Result<Vec<u8>, wasm_bindgen::JsError> {
//     Ok(self.0.to_bytes()?)
// }
#[no_mangle]
pub extern "C" fn txout_to_bytes(tx_out: *mut BSVTxOut) -> ByteArray {
    let tx_out = unsafe { &mut *tx_out };
    let bytes = tx_out.to_bytes().unwrap();
    let len = bytes.len();
    let out = bytes.into_boxed_slice();
    ByteArray { data: Box::into_raw(out) as *mut u8, len }
}

// pub fn to_hex(&self) -> Result<String, wasm_bindgen::JsError> {
//     Ok(self.0.to_hex()?)
// }
#[no_mangle]
pub extern "C" fn txout_to_hex(tx_out: *mut BSVTxOut) -> *mut libc::c_char {
    let tx_out = unsafe { &mut *tx_out };
    let string = tx_out.to_hex();
    let c_str = CString::new(string).unwrap();
    c_str.into_raw()
}

// pub fn to_json(&self) -> Result<JsValue, JsError> {
//     Ok(serde_wasm_bindgen::to_value(&self.0)?)
// }
#[no_mangle]
pub extern "C" fn txout_to_json(tx_out: *mut BSVTxOut) -> *mut c_char {
    let tx_out = unsafe { &mut *tx_out };
    let string = tx_out.to_json();
    let c_str = CString::new(string).unwrap();
    c_str.into_raw()
}

// pub fn to_json_string(&self) -> Result<String, wasm_bindgen::JsError> {
//     Ok(self.0.to_json_string()?)
// }
#[no_mangle]
pub extern "C" fn txout_to_json_string(tx_out: *mut BSVTxOut) -> *mut c_char {
    let tx_out = unsafe { &mut *tx_out };
    let string = tx_out.to_json_string();
    let c_str = CString::new(string).unwrap();
    c_str.into_raw()
}