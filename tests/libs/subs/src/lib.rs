extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn sub_i8(n1: i8, n2: i8) -> i8 {
    n1 - n2
}

#[wasm_bindgen]
pub fn sub_u8(n1: u8, n2: u8) -> u8 {
    n1 - n2
}

#[wasm_bindgen]
pub fn sub_i16(n1: i16, n2: i16) -> i16 {
    n1 - n2
}

#[wasm_bindgen]
pub fn sub_u16(n1: u16, n2: u16) -> u16 {
    n1 - n2
}

#[wasm_bindgen]
pub fn sub_i32(n1: i32, n2: i32) -> i32 {
    n1 - n2
}

#[wasm_bindgen]
pub fn sub_u32(n1: u32, n2: u32) -> u32 {
    n1 - n2
}

#[wasm_bindgen]
pub fn sub_i64(n1: i64, n2: i64) -> i64 {
    n1 - n2
}

#[wasm_bindgen]
pub fn sub_u64(n1: u64, n2: u64) -> u64 {
    n1 - n2
}
