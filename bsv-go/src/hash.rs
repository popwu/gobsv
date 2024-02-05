use bsv::Hash as BSVHash;

// pub fn to_bytes(&self) -> Vec<u8> {
//     self.0.to_bytes()
// }
#[no_mangle]
pub extern "C" fn hash_to_bytes(hash: *mut BSVHash) -> ByteArray {
    let hash = unsafe { &mut *hash };
    let bytes = hash.to_bytes();
    let len = bytes.len();
    let out = bytes.into_boxed_slice();
    ByteArray { data: Box::into_raw(out) as *mut u8, len }
}

// pub fn to_hex(&self) -> String {
//     self.0.to_hex()
// }
#[no_mangle]
pub extern "C" fn hash_to_hex(hash: *mut BSVHash) -> *mut libc::c_char {
    let hash = unsafe { &mut *hash };
    let hex = hash.to_hex();
    let c_str = CString::new(hex).unwrap();
    c_str.into_raw()
}

// pub fn sha_256d(input: &[u8]) -> Self {
//     Hash(BSVHash::sha_256d(input))
// }
#[no_mangle]
pub extern "C" fn sha_256d(input: *mut u8, len: usize) -> *mut BSVHash {
    let input = unsafe { std::slice::from_raw_parts(input, len) };
    let hash = BSVHash::sha_256d(input);
    Box::into_raw(Box::new(hash))
}

// pub fn sha_256(input: &[u8]) -> Self {
//     Hash(BSVHash::sha_256(input))
// }
#[no_mangle]
pub extern "C" fn sha_256(input: *mut u8, len: usize) -> *mut BSVHash {
    let input = unsafe { std::slice::from_raw_parts(input, len) };
    let hash = BSVHash::sha_256(input);
    Box::into_raw(Box::new(hash))
}

// pub fn sha_1(input: &[u8]) -> Self {
//     Hash(BSVHash::sha_1(input))
// }
#[no_mangle]
pub extern "C" fn sha_1(input: *mut u8, len: usize) -> *mut BSVHash {
    let input = unsafe { std::slice::from_raw_parts(input, len) };
    let hash = BSVHash::sha_1(input);
    Box::into_raw(Box::new(hash))
}

// pub fn ripemd_160(input: &[u8]) -> Self {
//     Hash(BSVHash::ripemd_160(input))
// }
#[no_mangle]
pub extern "C" fn ripemd_160(input: *mut u8, len: usize) -> *mut BSVHash {
    let input = unsafe { std::slice::from_raw_parts(input, len) };
    let hash = BSVHash::ripemd_160(input);
    Box::into_raw(Box::new(hash))
}

// pub fn hash_160(input: &[u8]) -> Self {
//     Hash(BSVHash::hash_160(input))
// }
#[no_mangle]
pub extern "C" fn hash_160(input: *mut u8, len: usize) -> *mut BSVHash {
    let input = unsafe { std::slice::from_raw_parts(input, len) };
    let hash = BSVHash::hash_160(input);
    Box::into_raw(Box::new(hash))
}

// pub fn sha_512(input: &[u8]) -> Self {
//     Hash(BSVHash::sha_512(input))
// }
#[no_mangle]
pub extern "C" fn sha_512(input: *mut u8, len: usize) -> *mut BSVHash {
    let input = unsafe { std::slice::from_raw_parts(input, len) };
    let hash = BSVHash::sha_512(input);
    Box::into_raw(Box::new(hash))
}

// pub fn sha_512_hmac(input: &[u8], key: &[u8]) -> Self {
//     Self(BSVHash::sha_256d_hmac(input, key))
// }
#[no_mangle]
pub extern "C" fn sha_512_hmac(input: *mut u8, input_len: usize, key: *mut u8, key_len: usize) -> *mut BSVHash {
    let input = unsafe { std::slice::from_raw_parts(input, input_len) };
    let key = unsafe { std::slice::from_raw_parts(key, key_len) };
    let hash = BSVHash::sha_512_hmac(input, key);
    Box::into_raw(Box::new(hash))
}

// pub fn sha_256_hmac(input: &[u8], key: &[u8]) -> Self {
//     Self(BSVHash::sha_256d_hmac(input, key))
// }
#[no_mangle]
pub extern "C" fn sha_256_hmac(input: *mut u8, input_len: usize, key: *mut u8, key_len: usize) -> *mut BSVHash {
    let input = unsafe { std::slice::from_raw_parts(input, input_len) };
    let key = unsafe { std::slice::from_raw_parts(key, key_len) };
    let hash = BSVHash::sha_256_hmac(input, key);
    Box::into_raw(Box::new(hash))
}

// pub fn sha_256d_hmac(input: &[u8], key: &[u8]) -> Self {
//     Self(BSVHash::sha_256d_hmac(input, key))
// }
#[no_mangle]
pub extern "C" fn sha_256d_hmac(input: *mut u8, input_len: usize, key: *mut u8, key_len: usize) -> *mut BSVHash {
    let input = unsafe { std::slice::from_raw_parts(input, input_len) };
    let key = unsafe { std::slice::from_raw_parts(key, key_len) };
    let hash = BSVHash::sha_256d_hmac(input, key);
    Box::into_raw(Box::new(hash))
}

// pub fn sha_1_hmac(input: &[u8], key: &[u8]) -> Self {
//     Self(BSVHash::sha_1_hmac(input, key))
// }
#[no_mangle]
pub extern "C" fn sha_1_hmac(input: *mut u8, input_len: usize, key: *mut u8, key_len: usize) -> *mut BSVHash {
    let input = unsafe { std::slice::from_raw_parts(input, input_len) };
    let key = unsafe { std::slice::from_raw_parts(key, key_len) };
    let hash = BSVHash::sha_1_hmac(input, key);
    Box::into_raw(Box::new(hash))
}

// pub fn ripemd_160_hmac(input: &[u8], key: &[u8]) -> Self {
//     Self(BSVHash::ripemd_160_hmac(input, key))
// }
#[no_mangle]
pub extern "C" fn ripemd_160_hmac(input: *mut u8, input_len: usize, key: *mut u8, key_len: usize) -> *mut BSVHash {
    let input = unsafe { std::slice::from_raw_parts(input, input_len) };
    let key = unsafe { std::slice::from_raw_parts(key, key_len) };
    let hash = BSVHash::ripemd_160_hmac(input, key);
    Box::into_raw(Box::new(hash))
}

// pub fn hash_160_hmac(input: &[u8], key: &[u8]) -> Self {
//     Self(BSVHash::hash_160_hmac(input, key))
// }
#[no_mangle]
pub extern "C" fn hash_160_hmac(input: *mut u8, input_len: usize, key: *mut u8, key_len: usize) -> *mut BSVHash {
    let input = unsafe { std::slice::from_raw_parts(input, input_len) };
    let key = unsafe { std::slice::from_raw_parts(key, key_len) };
    let hash = BSVHash::hash_160_hmac(input, key);
    Box::into_raw(Box::new(hash))
}