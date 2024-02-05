use bsv:: Transaction as BSVTransaction;
use crate::{
    keypair::private_key::PrivateKey,
    script::Script,
    sighash::{SigHash, SighashSignature},
    PublicKey,
};
use bsv::BSVTxIn as BSVBSVTxIn;
use bsv::BSVTxOut as BSVBSVTxOut;

use std::slice;
extern crate serde_json;
extern crate libc;
use libc::c_char;
use std::ffi::CString;

#[repr(C)]
pub struct ByteArray {
    data: *mut u8,
    len: usize,
}

#[repr(C)]
pub struct ByteArrayArray {
    data: *mut ByteArray,
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

// pub fn get_version(&self) -> u32 {
//     self.0.get_version()
// }
#[no_mangle]
pub extern "C" fn transaction_get_version(transaction: *mut BSVTransaction) -> u32 {
    let transaction = unsafe { &mut *transaction };
    transaction.get_version()
}

// pub fn get_ninputs(&self) -> usize {
//     self.0.get_ninputs()
// }
#[no_mangle]
pub extern "C" fn transaction_get_ninputs(transaction: *mut BSVTransaction) -> usize {
    let transaction = unsafe { &mut *transaction };
    transaction.get_ninputs()
}

// pub fn get_noutputs(&self) -> usize {
//     self.0.get_noutputs()
// }
#[no_mangle]
pub extern "C" fn transaction_get_noutputs(transaction: *mut BSVTransaction) -> usize {
    let transaction = unsafe { &mut *transaction };
    transaction.get_noutputs()
}

// pub fn get_input(&self, index: usize) -> Option<BSVTxIn> {
//     self.0.get_input(index).map(BSVTxIn)
// }
#[no_mangle]
pub extern "C" fn transaction_get_input(transaction: *mut BSVTransaction, index: usize) -> *mut BSVTxIn {
    let transaction = unsafe { &mut *transaction };
    let BSVTxIn = transaction.get_input(index).map(BSVTxIn);
    Box::into_raw(Box::new(BSVTxIn))
}

// pub fn get_output(&self, index: usize) -> Option<BSVTxOut> {
//     self.0.get_output(index).map(BSVTxOut)
// }
#[no_mangle]
pub extern "C" fn transaction_get_output(transaction: *mut BSVTransaction, index: usize) -> *mut BSVTxOut {
    let transaction = unsafe { &mut *transaction };
    let BSVTxOut = transaction.get_output(index).map(BSVTxOut);
    Box::into_raw(Box::new(BSVTxOut))
}

// pub fn get_n_locktime(&self) -> u32 {
//     self.0.get_n_locktime()
// }
#[no_mangle]
pub extern "C" fn transaction_get_n_locktime(transaction: *mut BSVTransaction) -> u32 {
    let transaction = unsafe { &mut *transaction };
    transaction.get_n_locktime()
}

// pub fn get_n_locktime_as_bytes(&self) -> Vec<u8> {
//     self.0.get_n_locktime_as_bytes()
// }
#[no_mangle]
pub extern "C" fn transaction_get_n_locktime_as_bytes(transaction: *mut BSVTransaction) -> ByteArray {
    let transaction = unsafe { &mut *transaction };
    let bytes = transaction.get_n_locktime_as_bytes();
    let len = bytes.len();
    let out = bytes.into_boxed_slice();
    ByteArray { data: Box::into_raw(out) as *mut u8, len }
}

// /**
//     * Creates a new empty transaction where you need to add inputs and outputs
//     * Transaction.add_input(BSVTxIn) and Transaction.add_output(BSVTxOut)
//     */
// #[wasm_bindgen(constructor)]
// pub fn new(version: u32, n_locktime: u32) -> Transaction {
//     Transaction(BSVTransaction::new(version, n_locktime))
// }
#[no_mangle]
pub extern "C" fn transaction_new(version: u32, n_locktime: u32) -> *mut BSVTransaction {
    let transaction = BSVTransaction::new(version, n_locktime);
    Box::into_raw(Box::new(transaction))
}

// pub fn set_version(&mut self, version: u32) -> Transaction {
//     Transaction(self.0.set_version(version))
// }
#[no_mangle]
pub extern "C" fn transaction_set_version(transaction: *mut BSVTransaction, version: u32) -> *mut BSVTransaction {
    let transaction = unsafe { &mut *transaction };
    let transaction = transaction.set_version(version);
    Box::into_raw(Box::new(transaction))
}

// pub fn set_nlocktime(&mut self, n_locktime: u32) -> Transaction {
//     Transaction(self.0.set_nlocktime(n_locktime))
// }
#[no_mangle]
pub extern "C" fn transaction_set_nlocktime(transaction: *mut BSVTransaction, n_locktime: u32) -> *mut BSVTransaction {
    let transaction = unsafe { &mut *transaction };
    let transaction = transaction.set_nlocktime(n_locktime);
    Box::into_raw(Box::new(transaction))
}

// pub fn add_input(&mut self, input: &BSVTxIn) {
//     self.0.add_input(&input.0)
// }
#[no_mangle]
pub extern "C" fn transaction_add_input(transaction: *mut BSVTransaction, input: *mut BSVTxIn) {
    let transaction = unsafe { &mut *transaction };
    let input = unsafe { &*input };
    transaction.add_input(input);
}

// pub fn prepend_input(&mut self, input: &BSVTxIn) {
//     self.0.prepend_input(&input.0)
// }
#[no_mangle]
pub extern "C" fn transaction_prepend_input(transaction: *mut BSVTransaction, input: *mut BSVTxIn) {
    let transaction = unsafe { &mut *transaction };
    let input = unsafe { &*input };
    transaction.prepend_input(input);
}

// pub fn insert_input(&mut self, index: usize, input: &BSVTxIn) {
//     self.0.insert_input(index, &input.0)
// }
#[no_mangle]
pub extern "C" fn transaction_insert_input(transaction: *mut BSVTransaction, index: usize, input: *mut BSVTxIn) {
    let transaction = unsafe { &mut *transaction };
    let input = unsafe { &*input };
    transaction.insert_input(index, input);
}

// pub fn add_output(&mut self, output: &BSVTxOut) {
//     self.0.add_output(&output.0)
// }
#[no_mangle]
pub extern "C" fn transaction_add_output(transaction: *mut BSVTransaction, output: *mut BSVTxOut) {
    let transaction = unsafe { &mut *transaction };
    let output = unsafe { &*output };
    transaction.add_output(output);
}

// pub fn prepend_output(&mut self, output: &BSVTxOut) {
//     self.0.prepend_output(&output.0)
// }
#[no_mangle]
pub extern "C" fn transaction_prepend_output(transaction: *mut BSVTransaction, output: *mut BSVTxOut) {
    let transaction = unsafe { &mut *transaction };
    let output = unsafe { &*output };
    transaction.prepend_output(output);
}

// pub fn insert_output(&mut self, index: usize, output: &BSVTxOut) {
//     self.0.insert_output(index, &output.0)
// }
#[no_mangle]
pub extern "C" fn transaction_insert_output(transaction: *mut BSVTransaction, index: usize, output: *mut BSVTxOut) {
    let transaction = unsafe { &mut *transaction };
    let output = unsafe { &*output };
    transaction.insert_output(index, output);
}

// pub fn set_input(&mut self, index: usize, input: &BSVTxIn) {
//     self.0.set_input(index, &input.0);
// }
#[no_mangle]
pub extern "C" fn transaction_set_input(transaction: *mut BSVTransaction, index: usize, input: *mut BSVTxIn) {
    let transaction = unsafe { &mut *transaction };
    let input = unsafe { &*input };
    transaction.set_input(index, input);
}

// pub fn set_output(&mut self, index: usize, output: &BSVTxOut) {
//     self.0.set_output(index, &output.0);
// }
#[no_mangle]
pub extern "C" fn transaction_set_output(transaction: *mut BSVTransaction, index: usize, output: *mut BSVTxOut) {
    let transaction = unsafe { &mut *transaction };
    let output = unsafe { &*output };
    transaction.set_output(index, output);
}

// pub fn is_coinbase_impl(&self) -> bool {
//     match (self.get_ninputs(), self.get_input(0)) {
//         (1, Some(x)) => x.0.is_coinbase(),
//         _ => false,
//     }
// }
#[no_mangle]
pub extern "C" fn transaction_is_coinbase_impl(transaction: *mut BSVTransaction) -> bool {
    let transaction = unsafe { &mut *transaction };
    match (transaction.get_ninputs(), transaction.get_input(0)) {
        (1, Some(x)) => x.is_coinbase(),
        _ => false,
    }
}

// /**
//     * XT Method:
//     * Returns the combined sum of all input satoshis.
//     * If any of the inputs dont have satoshis defined, this returns None or null
//     */
// pub fn satoshis_in(&self) -> Option<u64> {
//     self.0.satoshis_in()
// }
#[no_mangle]
pub extern "C" fn transaction_satoshis_in(transaction: *mut BSVTransaction) -> Option<u64> {
    let transaction = unsafe { &mut *transaction };
    transaction.satoshis_in()
}

// /**
//     * Returns the combined sum of all output satoshis.
//     */
// pub fn satoshis_out(&self) -> u64 {
//     self.0.satoshis_out()
// }
#[no_mangle]
pub extern "C" fn transaction_satoshis_out(transaction: *mut BSVTransaction) -> u64 {
    let transaction = unsafe { &mut *transaction };
    transaction.satoshis_out()
}

// pub fn from_hex(hex_str: &str) -> Result<Transaction, wasm_bindgen::JsError> {
//     Ok(Transaction(BSVTransaction::from_hex(hex_str)?))
// }
#[no_mangle]
pub extern "C" fn transaction_from_hex(hex_str: *mut c_char) -> *mut BSVTransaction {
    let hex_str = unsafe { CStr::from_ptr(hex_str) };
    let hex_str = hex_str.to_str().unwrap();
    let transaction = BSVTransaction::from_hex(hex_str).unwrap();
    Box::into_raw(Box::new(transaction))
}

// pub fn from_bytes(tx_bytes: &[u8]) -> Result<Transaction, wasm_bindgen::JsError> {
//     Ok(Transaction(BSVTransaction::from_bytes(tx_bytes)?))
// }
#[no_mangle]
pub extern "C" fn transaction_from_bytes(tx_bytes: *mut ByteArray) -> *mut BSVTransaction {
    let tx_bytes = unsafe { &*tx_bytes };
    let data = unsafe { std::slice::from_raw_parts(tx_bytes.data, tx_bytes.len as usize) };
    let transaction = BSVTransaction::from_bytes(data).unwrap();
    Box::into_raw(Box::new(transaction))
}

// pub fn to_json_string(&self) -> Result<String, wasm_bindgen::JsError> {
//     Ok(self.0.to_json_string()?)
// }
#[no_mangle]
pub extern "C" fn transaction_to_json_string(transaction: *mut BSVTransaction) -> *mut c_char {
    let transaction = unsafe { &mut *transaction };
    let json_string = transaction.to_json_string().unwrap();
    let c_str = CString::new(json_string).unwrap();
    c_str.into_raw()
}

// pub fn from_json_string(json_string: &str) -> Result<Transaction, wasm_bindgen::JsError> {
//     Ok(Transaction(BSVTransaction::from_json_string(json_string)?))
// }
#[no_mangle]
pub extern "C" fn transaction_from_json_string(json_string: *mut c_char) -> *mut BSVTransaction {
    let json_string = unsafe { CStr::from_ptr(json_string) };
    let json_string = json_string.to_str().unwrap();
    let transaction = BSVTransaction::from_json_string(json_string).unwrap();
    Box::into_raw(Box::new(transaction))
}

// pub fn to_json(&self) -> Result<JsValue, wasm_bindgen::JsError> {
//     Ok(serde_wasm_bindgen::to_value(&self.0)?)
// }
#[no_mangle]
pub extern "C" fn transaction_to_json(transaction: *mut BSVTransaction) -> *mut c_char {
    let transaction = unsafe { &mut *transaction };
    let json = serde_json::to_string(&transaction).unwrap();
    let c_str = CString::new(json).unwrap();
    c_str.into_raw()
}

// pub fn to_bytes(&self) -> Result<Vec<u8>, wasm_bindgen::JsError> {
//     Ok(self.0.to_bytes()?)
// }
#[no_mangle]
pub extern "C" fn transaction_to_bytes(transaction: *mut BSVTransaction) -> *mut ByteArray {
    let transaction = unsafe { &mut *transaction };
    let bytes = transaction.to_bytes().unwrap();
    let len = bytes.len();
    let out = bytes.into_boxed_slice();
    let byte_array = ByteArray { data: Box::into_raw(out) as *mut u8, len };
    Box::into_raw(Box::new(byte_array))
}

// pub fn to_hex(&self) -> Result<String, wasm_bindgen::JsError> {
//     Ok(self.0.to_hex()?)
// }
#[no_mangle]
pub extern "C" fn transaction_to_hex(transaction: *mut BSVTransaction) -> *mut c_char {
    let transaction = unsafe { &mut *transaction };
    let hex = transaction.to_hex().unwrap();
    let c_str = CString::new(hex).unwrap();
    c_str.into_raw()
}

// /**
//     * Get size of current serialised Transaction object
//     */
// pub fn get_size(&self) -> Result<usize, wasm_bindgen::JsError> {
//     Ok(self.0.get_size()?)
// }
#[no_mangle]
pub extern "C" fn transaction_get_size(transaction: *mut BSVTransaction) -> usize {
    let transaction = unsafe { &mut *transaction };
    transaction.get_size().unwrap()
}

// /**
//     * Adds an array of BSVTxIn's to the transaction
//     * @param {BSVTxIn[]} tx_ins
//     */
// pub fn add_inputs(&mut self, tx_ins: Vec<JsValue>) {
//     let js_value = &*tx_ins.to_vec();
//     for elem in js_value {
//         let input = serde_wasm_bindgen::from_value(elem.clone()).unwrap();
//         self.0.add_input(&input);
//     }
// }
#[no_mangle]
pub extern "C" fn transaction_add_inputs(transaction: *mut BSVTransaction, tx_ins: *mut BSVTxIn, len: usize) {
    let transaction = unsafe { &mut *transaction };
    let tx_ins = unsafe { std::slice::from_raw_parts(tx_ins, len) };
    for &tx_in in tx_ins {
        transaction.add_input(&tx_in);
    }
}

// /**
//     * Returns all outpoints from this transaction as a 2D array of 36 byte buffers.
//     *
//     * @returns {Uint8Array[]} outpoint_array
//     */
// pub fn get_outpoints(&mut self) -> Result<JsValue, wasm_bindgen::JsError> {
//     let outpoints = self.0.get_outpoints();
//     Ok(serde_wasm_bindgen::to_value(&outpoints)?)
// }
#[no_mangle]
pub extern "C" fn transaction_get_outpoints(transaction: *mut BSVTransaction) -> ByteArrayArray {
    let transaction = unsafe { &mut *transaction };
    let outpoints = transaction.get_outpoints();
    let mut byte_arrays: Vec<ByteArray> = outpoints.into_iter().map(|op| {
        let len = op.len();
        let data = op.into_boxed_slice().as_mut_ptr();
        ByteArray { data, len }
    }).collect();
    let len = byte_arrays.len();
    let data = byte_arrays.as_mut_ptr();
    std::mem::forget(byte_arrays);
    ByteArrayArray { data, len }
}

// /**
//     * Adds an array of BSVTxOuts to the transaction
//     * @param {BSVTxOut[]} tx_outs
//     */
// pub fn add_outputs(&mut self, tx_outs: Vec<JsValue>) {
//     let js_value = &*tx_outs.to_vec();
//     for elem in js_value {
//         let output = serde_wasm_bindgen::from_value(elem.clone()).unwrap();
//         self.0.add_output(&output);
//     }
// }


#[no_mangle]
pub extern "C" fn transaction_add_outputs(transaction: *mut BSVTransaction, tx_outs: *mut *mut BSVTxOut, len: usize) {
    let transaction = unsafe { &mut *transaction };
    let tx_outs = unsafe { slice::from_raw_parts(tx_outs, len) };
    for &output in tx_outs {
        let output = unsafe { &mut *output };
        transaction.add_output(output);
    }
}

// /**
//     * Gets the ID of the current transaction as a hex string.
//     */
// pub fn get_id_hex(&self) -> Result<String, wasm_bindgen::JsError> {
//     Ok(self.0.get_id_hex()?)
// }
#[no_mangle]
pub extern "C" fn transaction_get_id_hex(transaction: *mut BSVTransaction) -> *mut c_char {
    let transaction = unsafe { &mut *transaction };
    let id_hex = transaction.get_id_hex().unwrap();
    let c_str = CString::new(id_hex).unwrap();
    c_str.into_raw()
}

// /**
//     * Gets the ID of the current transaction as a Uint8Array.
//     */
// pub fn get_id_bytes(&self) -> Result<Vec<u8>, wasm_bindgen::JsError> {
//     Ok(self.0.get_id_bytes()?)
// }
#[no_mangle]
pub extern "C" fn transaction_get_id_bytes(transaction: *mut BSVTransaction) -> *mut ByteArray {
    let transaction = unsafe { &mut *transaction };
    let bytes = transaction.get_id_bytes().unwrap();
    let len = bytes.len();
    let out = bytes.into_boxed_slice();
    let byte_array = ByteArray { data: Box::into_raw(out) as *mut u8, len };
    Box::into_raw(Box::new(byte_array))
}

// /**
//     * Serialises this entire transaction to CBOR, preserving all fields from the standard Transaction format + TX+
//     */
// pub fn to_compact_bytes(&self) -> Result<Vec<u8>, wasm_bindgen::JsError> {
//     Ok(self.0.to_compact_bytes()?)
// }
#[no_mangle]
pub extern "C" fn transaction_to_compact_bytes(transaction: *mut BSVTransaction) -> *mut ByteArray {
    let transaction = unsafe { &mut *transaction };
    let bytes = transaction.to_compact_bytes().unwrap();
    let len = bytes.len();
    let out = bytes.into_boxed_slice();
    let byte_array = ByteArray { data: Box::into_raw(out) as *mut u8, len };
    Box::into_raw(Box::new(byte_array))
}

// pub fn to_compact_hex(&self) -> Result<String, wasm_bindgen::JsError> {
//     Ok(self.0.to_compact_hex()?)
// }
#[no_mangle]
pub extern "C" fn transaction_to_compact_hex(transaction: *mut BSVTransaction) -> *mut c_char {
    let transaction = unsafe { &mut *transaction };
    let hex = transaction.to_compact_hex().unwrap();
    let c_str = CString::new(hex).unwrap();
    c_str.into_raw()
}

// /**
//     * Deserialises the provided CBOR buffer to the TX+ format
//     */
// pub fn from_compact_bytes(compact_buffer: &[u8]) -> Result<Transaction, wasm_bindgen::JsError> {
//     Ok(Transaction(BSVTransaction::from_compact_bytes(compact_buffer)?))
// }
#[no_mangle]
pub extern "C" fn transaction_from_compact_bytes(data: *mut u8, len: usize) -> *mut BSVTransaction {
    let data = unsafe { std::slice::from_raw_parts(data, len) };
    let transaction = BSVTransaction::from_compact_bytes(data).unwrap();
    Box::into_raw(Box::new(transaction))
}

// /**
//     * Deserialises the provided CBOR buffer to the TX+ format
//     */
// pub fn from_compact_hex(compact_hex: String) -> Result<Transaction, wasm_bindgen::JsError> {
//     Ok(Transaction(BSVTransaction::from_compact_hex(&compact_hex)?))
// }
#[no_mangle]
pub extern "C" fn transaction_from_compact_hex(compact_hex: *mut c_char) -> *mut BSVTransaction {
    let compact_hex = unsafe { CStr::from_ptr(compact_hex) };
    let compact_hex = compact_hex.to_str().unwrap();
    let transaction = BSVTransaction::from_compact_hex(compact_hex).unwrap();
    Box::into_raw(Box::new(transaction))
}

// pub fn is_coinbase(&self) -> bool {
//     self.0.is_coinbase()
// }
#[no_mangle]
pub extern "C" fn transaction_is_coinbase(transaction: *mut BSVTransaction) -> bool {
    let transaction = unsafe { &mut *transaction };
    transaction.is_coinbase()
}

// pub fn sign(&mut self, priv_key: &PrivateKey, sighash: SigHash, n_tx_in: usize, unsigned_script: &Script, value: u64) -> Result<SighashSignature, wasm_bindgen::JsError> {
//     Ok(SighashSignature(self.0.sign(&priv_key.0, sighash.into(), n_tx_in, &unsigned_script.0, value)?))
// }
#[no_mangle]
pub extern "C" fn transaction_sign(transaction: *mut BSVTransaction, priv_key: *mut PrivateKey, sighash: SigHash, n_tx_in: usize, unsigned_script: *mut Script, value: u64) -> *mut SighashSignature {
    let transaction = unsafe { &mut *transaction };
    let priv_key = unsafe { &*priv_key };
    let unsigned_script = unsafe { &*unsigned_script };
    let sighash_signature = transaction.sign(priv_key, sighash, n_tx_in, unsigned_script, value).unwrap();
    Box::into_raw(Box::new(sighash_signature))
}

// pub fn sign_with_k(
//     &mut self,
//     priv_key: &PrivateKey,
//     ephemeral_key: &PrivateKey,
//     sighash: SigHash,
//     n_tx_in: usize,
//     unsigned_script: &Script,
//     value: u64,
// ) -> Result<SighashSignature, wasm_bindgen::JsError> {
//     Ok(SighashSignature(self.0.sign_with_k(
//         &priv_key.0,
//         &ephemeral_key.0,
//         sighash.into(),
//         n_tx_in,
//         &unsigned_script.0,
//         value,
//     )?))
// }
#[no_mangle]
pub extern "C" fn transaction_sign_with_k(transaction: *mut BSVTransaction, priv_key: *mut PrivateKey, ephemeral_key: *mut PrivateKey, sighash: SigHash, n_tx_in: usize, unsigned_script: *mut Script, value: u64) -> *mut SighashSignature {
    let transaction = unsafe { &mut *transaction };
    let priv_key = unsafe { &*priv_key };
    let ephemeral_key = unsafe { &*ephemeral_key };
    let unsigned_script = unsafe { &*unsigned_script };
    let sighash_signature = transaction.sign_with_k(priv_key, ephemeral_key, sighash, n_tx_in, unsigned_script, value).unwrap();
    Box::into_raw(Box::new(sighash_signature))
}

// pub fn sighash_preimage(&mut self, sighash: SigHash, n_tx_in: usize, unsigned_script: &Script, value: u64) -> Result<Vec<u8>, wasm_bindgen::JsError> {
//     Ok(self.0.sighash_preimage(sighash.into(), n_tx_in, &unsigned_script.0, value)?)
// }
#[no_mangle]
pub extern "C" fn transaction_sighash_preimage(transaction: *mut BSVTransaction, sighash: SigHash, n_tx_in: usize, unsigned_script: *mut Script, value: u64) -> *mut ByteArray {
    let transaction = unsafe { &mut *transaction };
    let unsigned_script = unsafe { &*unsigned_script };
    let preimage = transaction.sighash_preimage(sighash, n_tx_in, unsigned_script, value).unwrap();
    let len = preimage.len();
    let out = preimage.into_boxed_slice();
    let byte_array = ByteArray { data: Box::into_raw(out) as *mut u8, len };
    Box::into_raw(Box::new(byte_array))
}

// pub fn verify(&self, pub_key: &PublicKey, sig: &SighashSignature) -> bool {
//     self.0.verify(&pub_key.0, &sig.0)
// }
#[no_mangle]
pub extern "C" fn transaction_verify(transaction: *mut BSVTransaction, pub_key: *mut PublicKey, sig: *mut SighashSignature) -> bool {
    let transaction = unsafe { &mut *transaction };
    let pub_key = unsafe { &*pub_key };
    let sig = unsafe { &*sig };
    transaction.verify(pub_key, sig)
}

// pub fn _verify(&self, pub_key: &PublicKey, sig: &SighashSignature, reverse_digest: bool) -> bool {
//     self.0._verify(&pub_key.0, &sig.0, reverse_digest)
// }
#[no_mangle]
pub extern "C" fn transaction__verify(transaction: *mut BSVTransaction, pub_key: *mut PublicKey, sig: *mut SighashSignature, reverse_digest: bool) -> bool {
    let transaction = unsafe { &mut *transaction };
    let pub_key = unsafe { &*pub_key };
    let sig = unsafe { &*sig };
    transaction._verify(pub_key, sig, reverse_digest)
}
