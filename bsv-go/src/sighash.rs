use crate::signature::Signature;
use bsv::SighashSignature as BSVSighashSignature;

// #[wasm_bindgen(constructor)]
// pub fn new(signature: &Signature, sighash_type: SigHash, sighash_buffer: &[u8]) -> SighashSignature {
//     SighashSignature(BSVSighashSignature::new(&signature.0, sighash_type.into(), sighash_buffer))
// }
#[no_mangle]
pub extern "C" fn sighash_signature_new(signature: *mut BSVSignature, sighash_type: u8, sighash_buffer: &[u8]) -> *mut BSVSighashSignature {
    let signature = unsafe { &mut *signature };
    let sighash_type = SigHash::from_u8(sighash_type).unwrap(); // 假设 SigHash::from_u8 是一个存在的函数
    let sighash_signature = BSVSighashSignature::new(&signature, sighash_type, sighash_buffer);
    Box::into_raw(Box::new(sighash_signature))
}

// pub fn to_hex(&self) -> Result<String, wasm_bindgen::JsError> {
//     Ok(BSVSighashSignature::to_hex(&self.0)?)
// }
#[no_mangle]
pub extern "C" fn sighash_signature_to_hex(sighash_signature: *mut BSVSighashSignature) -> *mut libc::c_char {
    let sighash_signature = unsafe { &mut *sighash_signature };
    let hex = BSVSighashSignature::to_hex(&sighash_signature);
    let c_str = CString::new(hex).unwrap();
    c_str.into_raw()
}

// pub fn to_bytes(&self) -> Result<Vec<u8>, wasm_bindgen::JsError> {
//     Ok(BSVSighashSignature::to_bytes(&self.0)?)
// }
#[no_mangle]
pub extern "C" fn sighash_signature_to_bytes(sighash_signature: *mut BSVSighashSignature) -> ByteArray {
    let sighash_signature = unsafe { &mut *sighash_signature };
    let bytes = BSVSighashSignature::to_bytes(&sighash_signature);
    match bytes {
        Ok(bytes) => {
            let len = bytes.len();
            let out = bytes.into_boxed_slice();
            ByteArray { data: Box::into_raw(out) as *mut u8, len }
        },
        Err(_) => ByteArray { data: std::ptr::null_mut(), len: 0 }, // 返回空 ByteArray，如果有错误
    }
}

// pub fn from_bytes(bytes: &[u8], sighash_buffer: &[u8]) -> Result<SighashSignature, wasm_bindgen::JsError> {
//     Ok(SighashSignature(BSVSighashSignature::from_bytes(bytes, sighash_buffer)?))
// }
#[no_mangle]
pub extern "C" fn sighash_signature_from_bytes(bytes: &[u8], sighash_buffer: &[u8]) -> *mut BSVSighashSignature {
    let sighash_signature = BSVSighashSignature::from_bytes(bytes, sighash_buffer);
    match sighash_signature {
        Ok(signature) => Box::into_raw(Box::new(signature)),
        Err(_) => std::ptr::null_mut(), // 返回空指针，如果有错误
    }
}