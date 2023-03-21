
use wasm_bindgen::prelude::*;

const MEM_SIZE: usize = 100;
static mut MEM: [u8;MEM_SIZE] = [1;MEM_SIZE];

#[wasm_bindgen]
pub fn add() {
    unsafe{
        MEM[0] += 10;
        MEM[1] += 20;
    }
}

#[wasm_bindgen]
pub fn get_ptr() -> *const u8 {
    let ptr;
    unsafe {
        ptr = MEM.as_ptr();
    }
    ptr
}

#[wasm_bindgen]
pub fn swap() {
    unsafe{
        let temp = MEM[0];
        MEM[0] = MEM[1];
        MEM[1] = temp;
    }
}

