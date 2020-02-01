// use wasm_bindgen::prelude::*;
// use wasm_bindgen::JsCast;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct PClass {
    pub name: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FunctionIO {
    pub name: String,
    pub label: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct FunctionInterface {
    pub index: u32,
    pub name: String,
    pub inputs: Vec<FunctionIO>,
    pub outputs: Vec<FunctionIO>,
    pub pclass: PClass,
}

pub type GraphSteps = Vec<Vec<FunctionInterface>>;
