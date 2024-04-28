extern crate js_sys;
extern crate wasm_bindgen;
extern crate mpcemu_v53;

use wasm_bindgen::prelude::*;

use js_sys::{
    Uint8Array,
    Error
};

#[wasm_bindgen]
pub struct V53(mpcemu_v53::CPU);

#[wasm_bindgen]
impl V53 {
    #[wasm_bindgen(constructor)]
    pub fn new (rom: Uint8Array) -> Result<V53, Error> {
        console_error_panic_hook::set_once();
        let mut mem: Vec<u8> = vec![0u8; rom.length() as usize];
        rom.copy_to(&mut mem);
        Ok(Self(mpcemu_v53::CPU::new(mem)))
    }

    #[wasm_bindgen]
    pub fn step () {
        unimplemented!()
    }

    #[wasm_bindgen]
    pub fn read_state () {
        unimplemented!()
    }

    #[wasm_bindgen]
    pub fn read_memory () {
        unimplemented!()
    }
}
