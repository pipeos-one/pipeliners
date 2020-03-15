extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn fibonacci_i8(n: i8) -> i8 {
    match n {
      1 | 2 => 1,
      n => fibonacci_i8(n - 1) + fibonacci_i8(n - 2)
  }
}

#[wasm_bindgen]
pub fn fibonacci_u8(n: u8) -> u8 {
    match n {
      1 | 2 => 1,
      n => fibonacci_u8(n - 1) + fibonacci_u8(n - 2)
  }
}

#[wasm_bindgen]
pub fn fibonacci_i16(n: i16) -> i16 {
    match n {
      1 | 2 => 1,
      n => fibonacci_i16(n - 1) + fibonacci_i16(n - 2)
  }
}

#[wasm_bindgen]
pub fn fibonacci_u16(n: u16) -> u16 {
    match n {
      1 | 2 => 1,
      n => fibonacci_u16(n - 1) + fibonacci_u16(n - 2)
  }
}

#[wasm_bindgen]
pub fn fibonacci_i32(n: i32) -> i32 {
    match n {
      1 | 2 => 1,
      n => fibonacci_i32(n - 1) + fibonacci_i32(n - 2)
  }
}

#[wasm_bindgen]
pub fn fibonacci_u32(n: u32) -> u32 {
    match n {
      1 | 2 => 1,
      n => fibonacci_u32(n - 1) + fibonacci_u32(n - 2)
  }
}

#[wasm_bindgen]
pub fn fibonacci_i64(n: i64) -> i64 {
    match n {
      1 | 2 => 1,
      n => fibonacci_i64(n - 1) + fibonacci_i64(n - 2)
  }
}

#[wasm_bindgen]
pub fn fibonacci_u64(n: u64) -> u64 {
    match n {
      1 | 2 => 1,
      n => fibonacci_u64(n - 1) + fibonacci_u64(n - 2)
  }
}
