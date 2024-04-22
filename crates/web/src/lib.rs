extern crate wasm_bindgen;
extern crate mpcemu_v53;

use js_sys::{
    Uint8Array,
    Error
};

#[wasm_bindgen]
pub struct V53(mpcemu_v53::V53);

#[wasm_bindgen]
impl V53 {
    #[wasm_bindgen(constructor)]
    pub async fn new (rom: Uint8Array) -> Result<API, Error> {
        console_error_panic_hook::set_once();
        let mem: Vec<u8> = vec![0u8; rom.length() as usize];
        rom.copy_to(mem);
        Ok(Self(V53::new(mem)))
    }

    #[wasm_bindgen]
    fn step () {
        unimplemented!()
    }

    #[wasm_bindgen]
    fn read_state () {
        unimplemented!()
    }

    #[wasm_bindgen]
    fn read_memory () {
        unimplemented!()
    }
}
