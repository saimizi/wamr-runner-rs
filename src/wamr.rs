#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(unused)]
mod libiwasm;

use libiwasm::*;

pub struct Wamr {}

impl Wamr {
    pub fn init() -> bool {
        unsafe { libiwasm::wasm_runtime_init() }
    }
}
