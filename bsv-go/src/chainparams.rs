use bsv::chainparams::ChainParams as BSVChainParams;

// #[wasm_bindgen(constructor)]
// pub fn new() -> ChainParams {
//     ChainParams(BSVChainParams::default())
// }
#[no_mangle]
pub extern "C" fn chainparams_new() -> *mut BSVChainParams {
    let chain_params = BSVChainParams::default();
    Box::into_raw(Box::new(chain_params))
}

// pub fn mainnet() -> ChainParams {
//     ChainParams(BSVChainParams::default())
// }
#[no_mangle]
pub extern "C" fn chainparams_mainnet() -> *mut BSVChainParams {
    let chain_params = BSVChainParams::default();
    Box::into_raw(Box::new(chain_params))
}

// pub fn testnet() -> ChainParams {
//     ChainParams(BSVChainParams::testnet())
// }
#[no_mangle]
pub extern "C" fn chainparams_testnet() -> *mut BSVChainParams {
    let chain_params = BSVChainParams::testnet();
    Box::into_raw(Box::new(chain_params))
}

// pub fn regtest() -> ChainParams {
//     ChainParams(BSVChainParams::regtest())
// }
#[no_mangle]
pub extern "C" fn chainparams_regtest() -> *mut BSVChainParams {
    let chain_params = BSVChainParams::regtest();
    Box::into_raw(Box::new(chain_params))
}

// pub fn stn() -> ChainParams {
//     ChainParams(BSVChainParams::stn())
// }
#[no_mangle]
pub extern "C" fn chainparams_stn() -> *mut BSVChainParams {
    let chain_params = BSVChainParams::stn();
    Box::into_raw(Box::new(chain_params))
}