use bsv::Script;
use bsv::Status;
use bsv::Transaction;
use bsv::{Interpreter as BSVInterpreter, State as BSVState};

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

// Interpreter

// pub fn from_transaction(tx: Transaction, txin_idx: usize) -> Result<Interpreter, JsError> {
//     Ok(Interpreter(BSVInterpreter::from_transaction(&tx.0, txin_idx)?))
// }
#[no_mangle]
pub extern "C" fn interpreter_from_transaction(
    tx: *mut Transaction,
    txin_idx: usize,
) -> *mut BSVInterpreter {
    let tx = unsafe { &mut *tx };
    let interpreter = BSVInterpreter::from_transaction(&tx, txin_idx).unwrap();
    Box::into_raw(Box::new(interpreter))
}

// pub fn from_script(script: Script) -> Interpreter {
//     Interpreter(BSVInterpreter::from_script(&script.0))
// }
#[no_mangle]
pub extern "C" fn interpreter_from_script(script: *mut Script) -> *mut BSVInterpreter {
    let script = unsafe { &mut *script };
    let interpreter = BSVInterpreter::from_script(&script);
    Box::into_raw(Box::new(interpreter))
}

// pub fn run(&mut self) -> Result<(), JsError> {
//     Ok(self.0.run()?)
// }
#[no_mangle]
pub extern "C" fn interpreter_run(interpreter: *mut BSVInterpreter) {
    let interpreter = unsafe { &mut *interpreter };
    interpreter.run().unwrap();
}

// #[wasm_bindgen(js_name = "next")]
// pub fn step(&mut self) -> Result<Option<State>, JsError> {
//     let state = match self.0.next() {
//         Some(v) => v?,
//         None => return Ok(None),
//     };
//     let js_state = State(state);
//     Ok(Some(js_state))
// }
#[no_mangle]
pub extern "C" fn interpreter_step(interpreter: *mut BSVInterpreter) -> *mut BSVState {
    let interpreter = unsafe { &mut *interpreter };
    let state = match interpreter.next() {
        Some(v) => v.unwrap(),
        None => return std::ptr::null_mut(),
    };
    Box::into_raw(Box::new(state))
}

// pub fn get_state(&self) -> State {
//     State(self.0.state())
// }
#[no_mangle]
pub extern "C" fn interpreter_get_state(interpreter: *mut BSVInterpreter) -> *mut BSVState {
    let interpreter = unsafe { &mut *interpreter };
    let state = interpreter.state();
    Box::into_raw(Box::new(state))
}

// State

// pub fn get_executed_script(&self) -> Result<Script, JsError> {
//     let asm_string: String = self.0.executed_opcodes.iter().map(|x| x.to_string()).fold(String::new(), |acc, x| format!("{} {}", acc, x));
//     Script::from_asm_string(&asm_string)
// }
#[no_mangle]
pub extern "C" fn state_get_executed_script(state: *mut BSVState) -> *mut Script {
    let state = unsafe { &mut *state };
    let asm_string: String = state
        .executed_opcodes
        .iter()
        .map(|x| x.to_string())
        .fold(String::new(), |acc, x| format!("{} {}", acc, x));
    let script = Script::from_asm_string(&asm_string).unwrap();
    Box::into_raw(Box::new(script))
}

// pub fn get_stack(&self) -> Result<JsValue, JsError> {
//     let stack = self.0.stack();
//     Ok(serde_wasm_bindgen::to_value(stack)?)
// }
#[no_mangle]
pub extern "C" fn state_get_stack(state: *mut BSVState) -> *mut ByteArray {
    let state = unsafe { &mut *state };
    let stack = state.stack();
    let c_stack: Vec<ByteArray> = stack.iter().map(|v| ByteArray::from(v.clone())).collect();
    let ptr = c_stack.as_ptr() as *mut ByteArray;
    std::mem::forget(c_stack);
    ptr
}

// pub fn get_alt_stack(&self) -> Result<JsValue, JsError> {
//     let stack = &self.0.alt_stack;
//     Ok(serde_wasm_bindgen::to_value(&stack)?)
// }
#[no_mangle]
pub extern "C" fn state_get_alt_stack(state: *mut BSVState) -> *mut ByteArray {
    let state = unsafe { &mut *state };
    let stack = &state.alt_stack;
    let c_stack: Vec<ByteArray> = stack.iter().map(|v| ByteArray::from(v.clone())).collect();
    let ptr = c_stack.as_ptr() as *mut ByteArray;
    std::mem::forget(c_stack);
    ptr
}

// pub fn get_status(&self) -> JsStatus {
//     self.0.status.clone().into()
// }
#[no_mangle]
pub extern "C" fn state_get_status(state: *mut BSVState) -> i32 {
    let state = unsafe { &mut *state };
    match state.status {
        Status::Running => 0,
        Status::Finished => 1,
    }
}
