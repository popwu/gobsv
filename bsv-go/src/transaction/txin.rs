use bsv::Script as BSVScript;
use bsv::TxIn as BSVTxIn;
use std::ffi::CStr;
use std::ffi::CString;
use std::os::raw::c_char;
extern crate serde_json;

#[repr(C)]
pub struct ByteArray {
    data: *mut u8,
    len: usize,
}

impl From<Vec<u8>> for ByteArray {
    fn from(vec: Vec<u8>) -> Self {
        let len = vec.len();
        let data = vec.as_ptr() as *mut u8;
        std::mem::forget(vec);
        Self { data, len }
    }
}

// #[wasm_bindgen(constructor)]
// pub fn new(prev_tx_id: &[u8], vout: u32, unlocking_script: &Script, sequence: Option<u32>) -> TxIn {
//     TxIn(BSVTxIn::new(prev_tx_id, vout, &unlocking_script.0, sequence))
// }
#[no_mangle]
pub extern "C" fn txin_new(
    prev_tx_id: *mut u8,
    prev_tx_id_len: usize,
    vout: u32,
    unlocking_script: *mut BSVScript,
    sequence: Option<u32>,
) -> *mut BSVTxIn {
    let prev_tx_id = unsafe { std::slice::from_raw_parts(prev_tx_id, prev_tx_id_len) };
    let unlocking_script = unsafe { &*unlocking_script };
    let txin = BSVTxIn::new(prev_tx_id, vout, &unlocking_script, sequence);
    Box::into_raw(Box::new(txin))
}

// pub fn empty() -> TxIn {
//     TxIn::default()
// }
#[no_mangle]
pub extern "C" fn txin_empty() -> *mut BSVTxIn {
    let txin = BSVTxIn::default();
    Box::into_raw(Box::new(txin))
}

// pub fn get_prev_tx_id(&self, little_endian: Option<bool>) -> Vec<u8> {
//     self.0.get_prev_tx_id(little_endian)
// }
#[no_mangle]
pub extern "C" fn txin_get_prev_tx_id(
    txin: *mut BSVTxIn,
    little_endian: Option<bool>,
) -> *mut ByteArray {
    let txin = unsafe { &*txin };
    let prev_tx_id = txin.get_prev_tx_id(little_endian);
    let data = prev_tx_id.as_ptr() as *mut u8;
    let len = prev_tx_id.len();
    std::mem::forget(prev_tx_id);
    Box::into_raw(Box::new(ByteArray { data, len }))
}

// pub fn get_prev_tx_id_hex(&self, little_endian: Option<bool>) -> String {
//     self.0.get_prev_tx_id_hex(little_endian)
// }
#[no_mangle]
pub extern "C" fn txin_get_prev_tx_id_hex(
    txin: *mut BSVTxIn,
    little_endian: Option<bool>,
) -> *mut libc::c_char {
    let txin = unsafe { &*txin };
    let prev_tx_id_hex = txin.get_prev_tx_id_hex(little_endian);
    let c_str = CString::new(prev_tx_id_hex).unwrap();
    c_str.into_raw()
}

// pub fn get_vout(&self) -> u32 {
//     self.0.get_vout()
// }
#[no_mangle]
pub extern "C" fn txin_get_vout(txin: *mut BSVTxIn) -> u32 {
    let txin = unsafe { &*txin };
    txin.get_vout()
}

// pub fn get_unlocking_script_size(&self) -> u64 {
//     self.0.get_unlocking_script_size()
// }
#[no_mangle]
pub extern "C" fn txin_get_unlocking_script_size(txin: *mut BSVTxIn) -> u64 {
    let txin = unsafe { &*txin };
    txin.get_unlocking_script_size()
}

// pub fn get_unlocking_script(&self) -> Script {
//     Script(self.0.get_unlocking_script())
// }
#[no_mangle]
pub extern "C" fn txin_get_unlocking_script(txin: *mut BSVTxIn) -> *mut BSVScript {
    let txin = unsafe { &*txin };
    let unlocking_script = txin.get_unlocking_script();
    Box::into_raw(Box::new(unlocking_script))
}

// pub fn get_unlocking_script_hex(&self) -> String {
//     self.0.get_unlocking_script_hex()
// }
#[no_mangle]
pub extern "C" fn txin_get_unlocking_script_hex(txin: *mut BSVTxIn) -> *mut libc::c_char {
    let txin = unsafe { &*txin };
    let unlocking_script_hex = txin.get_unlocking_script_hex();
    let c_str = CString::new(unlocking_script_hex).unwrap();
    c_str.into_raw()
}

// pub fn get_sequence(&self) -> u32 {
//     self.0.get_sequence()
// }
#[no_mangle]
pub extern "C" fn txin_get_sequence(txin: *mut BSVTxIn) -> u32 {
    let txin = unsafe { &*txin };
    txin.get_sequence()
}

// pub fn get_sequence_as_bytes(&self) -> Vec<u8> {
//     self.0.get_sequence_as_bytes()
// }
#[no_mangle]
pub extern "C" fn txin_get_sequence_as_bytes(txin: *mut BSVTxIn) -> *mut ByteArray {
    let txin = unsafe { &*txin };
    let sequence_as_bytes = txin.get_sequence_as_bytes();
    let data = sequence_as_bytes.as_ptr() as *mut u8;
    let len = sequence_as_bytes.len();
    std::mem::forget(sequence_as_bytes);
    Box::into_raw(Box::new(ByteArray { data, len }))
}

// pub fn get_outpoint_bytes(&self, little_endian: Option<bool>) -> Vec<u8> {
//     self.0.get_outpoint_bytes(little_endian)
// }
#[no_mangle]
pub extern "C" fn txin_get_outpoint_bytes(
    txin: *mut BSVTxIn,
    little_endian: Option<bool>,
) -> *mut ByteArray {
    let txin = unsafe { &*txin };
    let outpoint_bytes = txin.get_outpoint_bytes(little_endian);
    let data = outpoint_bytes.as_ptr() as *mut u8;
    let len = outpoint_bytes.len();
    std::mem::forget(outpoint_bytes);
    Box::into_raw(Box::new(ByteArray { data, len }))
}

// pub fn get_outpoint_hex(&self, little_endian: Option<bool>) -> String {
//     self.0.get_outpoint_hex(little_endian)
// }
#[no_mangle]
pub extern "C" fn txin_get_outpoint_hex(
    txin: *mut BSVTxIn,
    little_endian: Option<bool>,
) -> *mut libc::c_char {
    let txin = unsafe { &*txin };
    let outpoint_hex = txin.get_outpoint_hex(little_endian);
    let c_str = CString::new(outpoint_hex).unwrap();
    c_str.into_raw()
}

// pub fn set_unlocking_script(&mut self, script: &Script) {
//     self.0.set_unlocking_script(&script.0)
// }
#[no_mangle]
pub extern "C" fn txin_set_unlocking_script(txin: *mut BSVTxIn, script: *mut BSVScript) {
    let txin = unsafe { &mut *txin };
    let script = unsafe { &*script };
    txin.set_unlocking_script(&script);
}

// pub fn set_prev_tx_id(&mut self, txid: &[u8]) {
//     self.0.set_prev_tx_id(txid)
// }
#[no_mangle]
pub extern "C" fn txin_set_prev_tx_id(txin: *mut BSVTxIn, txid: *mut u8, len: usize) {
    let txin = unsafe { &mut *txin };
    let txid = unsafe { std::slice::from_raw_parts(txid, len) };
    txin.set_prev_tx_id(txid);
}

// pub fn set_vout(&mut self, vout: u32) {
//     self.0.set_vout(vout);
// }
#[no_mangle]
pub extern "C" fn txin_set_vout(txin: *mut BSVTxIn, vout: u32) {
    let txin = unsafe { &mut *txin };
    txin.set_vout(vout);
}

// pub fn set_sequence(&mut self, sequence: u32) {
//     self.0.set_sequence(sequence);
// }
#[no_mangle]
pub extern "C" fn txin_set_sequence(txin: *mut BSVTxIn, sequence: u32) {
    let txin = unsafe { &mut *txin };
    txin.set_sequence(sequence);
}

// pub fn set_satoshis(&mut self, satoshis: u64) {
//     self.0.set_satoshis(satoshis);
// }
#[no_mangle]
pub extern "C" fn txin_set_satoshis(txin: *mut BSVTxIn, satoshis: u64) {
    let txin = unsafe { &mut *txin };
    txin.set_satoshis(satoshis);
}

// pub fn get_satoshis(&self) -> Option<u64> {
//     self.0.get_satoshis()
// }
#[no_mangle]
pub extern "C" fn txin_get_satoshis(txin: *mut BSVTxIn) -> Option<u64> {
    let txin = unsafe { &*txin };
    txin.get_satoshis()
}

// pub fn set_locking_script(&mut self, locking_script: &Script) {
//     self.0.set_locking_script(&locking_script.0)
// }
#[no_mangle]
pub extern "C" fn txin_set_locking_script(txin: *mut BSVTxIn, locking_script: *mut BSVScript) {
    let txin = unsafe { &mut *txin };
    let locking_script = unsafe { &*locking_script };
    txin.set_locking_script(&locking_script);
}

// pub fn get_locking_script(&self) -> Option<Script> {
//     self.0.get_locking_script().map(Script)
// }
#[no_mangle]
pub extern "C" fn txin_get_locking_script(txin: *mut BSVTxIn) -> *mut BSVScript {
    let txin = unsafe { &*txin };
    let locking_script: Option<BSVScript> = txin.get_locking_script();
    Box::into_raw(Box::new(locking_script.unwrap_or_else(BSVScript::default)))
}

// pub fn get_locking_script_bytes(&self) -> Option<Vec<u8>> {
//     self.0.get_locking_script_bytes()
// }
#[no_mangle]
pub extern "C" fn txin_get_locking_script_bytes(txin: *mut BSVTxIn) -> *mut ByteArray {
    let txin = unsafe { &*txin };
    if let Some(locking_script_bytes) = txin.get_locking_script_bytes() {
        let data = locking_script_bytes.as_ptr() as *mut u8;
        let len = locking_script_bytes.len();
        std::mem::forget(locking_script_bytes);
        Box::into_raw(Box::new(ByteArray { data, len }))
    } else {
        std::ptr::null_mut()
    }
}

// pub fn from_hex(hex_str: &str) -> Result<TxIn, wasm_bindgen::JsError> {
//     Ok(TxIn(BSVTxIn::from_hex(hex_str)?))
// }
#[no_mangle]
pub extern "C" fn txin_from_hex(hex_str: *mut libc::c_char) -> *mut BSVTxIn {
    let hex_str = unsafe { CStr::from_ptr(hex_str) };
    let hex_str = hex_str.to_str().unwrap();
    let txin = BSVTxIn::from_hex(hex_str).unwrap();
    Box::into_raw(Box::new(txin))
}

// pub fn to_json(&self) -> Result<JsValue, JsError> {
//     Ok(serde_wasm_bindgen::to_value(&self.0)?)
// }
#[no_mangle]
pub extern "C" fn txin_to_json(txin: *mut BSVTxIn) -> *mut c_char {
    let txin = unsafe { &*txin };
    let json = serde_json::to_string(&txin).unwrap();
    let c_string = CString::new(json).unwrap();
    c_string.into_raw()
}

// pub fn to_json_string(&self) -> Result<String, wasm_bindgen::JsError> {
//     Ok(self.0.to_json_string()?)
// }
#[no_mangle]
pub extern "C" fn txin_to_json_string(txin: *mut BSVTxIn) -> *mut libc::c_char {
    let txin = unsafe { &*txin };
    let json_string = txin.to_json_string().unwrap();
    let c_str = CString::new(json_string).unwrap();
    c_str.into_raw()
}

// pub fn to_bytes(&self) -> Result<Vec<u8>, wasm_bindgen::JsError> {
//     Ok(self.0.to_bytes()?)
// }
#[no_mangle]
pub extern "C" fn txin_to_bytes(txin: *mut BSVTxIn) -> *mut ByteArray {
    let txin = unsafe { &*txin };
    let bytes = txin.to_bytes().unwrap();
    let data = bytes.as_ptr() as *mut u8;
    let len = bytes.len();
    std::mem::forget(bytes);
    Box::into_raw(Box::new(ByteArray { data, len }))
}

// pub fn to_hex(&self) -> Result<String, wasm_bindgen::JsError> {
//     Ok(self.0.to_hex()?)
// }
#[no_mangle]
pub extern "C" fn txin_to_hex(txin: *mut BSVTxIn) -> *mut libc::c_char {
    let txin = unsafe { &*txin };
    let hex = txin.to_hex().unwrap();
    let c_str = CString::new(hex).unwrap();
    c_str.into_raw()
}

// pub fn from_outpoint_bytes(outpoint: &[u8]) -> Result<TxIn, wasm_bindgen::JsError> {
//     Ok(TxIn(BSVTxIn::from_outpoint_bytes(outpoint)?))
// }
#[no_mangle]
pub extern "C" fn txin_from_outpoint_bytes(outpoint: *mut u8, len: usize) -> *mut BSVTxIn {
    let outpoint = unsafe { std::slice::from_raw_parts(outpoint, len) };
    let txin = BSVTxIn::from_outpoint_bytes(outpoint).unwrap();
    Box::into_raw(Box::new(txin))
}

// /**
//     * Serialises this entire transaction to CBOR, preserving all fields from the standard Transaction format + TX+
//     */
// pub fn to_compact_bytes(&self) -> Result<Vec<u8>, wasm_bindgen::JsError> {
//     Ok(self.0.to_compact_bytes()?)
// }
#[no_mangle]
pub extern "C" fn txin_to_compact_bytes(txin: *mut BSVTxIn) -> *mut ByteArray {
    let txin = unsafe { &*txin };
    let bytes = txin.to_compact_bytes().unwrap();
    let data = bytes.as_ptr() as *mut u8;
    let len = bytes.len();
    std::mem::forget(bytes);
    Box::into_raw(Box::new(ByteArray { data, len }))
}

// pub fn to_compact_hex(&self) -> Result<String, wasm_bindgen::JsError> {
//     Ok(self.0.to_compact_hex()?)
// }
#[no_mangle]
pub extern "C" fn txin_to_compact_hex(txin: *mut BSVTxIn) -> *mut libc::c_char {
    let txin = unsafe { &*txin };
    let hex = txin.to_compact_hex().unwrap();
    let c_str = CString::new(hex).unwrap();
    c_str.into_raw()
}

// /**
//     * Deserialises the provided CBOR buffer to the TX+ format
//     */
// pub fn from_compact_bytes(compact_buffer: &[u8]) -> Result<TxIn, wasm_bindgen::JsError> {
//     Ok(TxIn(BSVTxIn::from_compact_bytes(compact_buffer)?))
// }
#[no_mangle]
pub extern "C" fn txin_from_compact_bytes(compact_buffer: *mut u8, len: usize) -> *mut BSVTxIn {
    let compact_buffer = unsafe { std::slice::from_raw_parts(compact_buffer, len) };
    let txin = BSVTxIn::from_compact_bytes(compact_buffer).unwrap();
    Box::into_raw(Box::new(txin))
}

// /**
//     * Deserialises the provided CBOR buffer to the TX+ format
//     */
// pub fn from_compact_hex(compact_hex: String) -> Result<TxIn, wasm_bindgen::JsError> {
//     Ok(TxIn(BSVTxIn::from_compact_hex(&compact_hex)?))
// }
#[no_mangle]
pub extern "C" fn txin_from_compact_hex(compact_hex: *mut libc::c_char) -> *mut BSVTxIn {
    let compact_hex = unsafe { CStr::from_ptr(compact_hex) };
    let compact_hex = compact_hex.to_str().unwrap();
    let txin = BSVTxIn::from_compact_hex(compact_hex).unwrap();
    Box::into_raw(Box::new(txin))
}

// /// Concatenates ScriptSig and UnlockingScript into a single script.
// pub fn get_finalised_script(&self) -> Result<Script, JsError> {
//     Ok(Script(self.0.get_finalised_script()?))
// }
#[no_mangle]
pub extern "C" fn txin_get_finalised_script(txin: *mut BSVTxIn) -> *mut BSVScript {
    let txin = unsafe { &*txin };
    let finalised_script = txin.get_finalised_script().unwrap();
    Box::into_raw(Box::new(finalised_script))
}

// // Checks if input is a coinbase
// pub fn is_coinbase(&self) -> bool {
//     self.0.is_coinbase()
// }
#[no_mangle]
pub extern "C" fn txin_is_coinbase(txin: *mut BSVTxIn) -> bool {
    let txin = unsafe { &*txin };
    txin.is_coinbase()
}
