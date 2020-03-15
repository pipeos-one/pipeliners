// // retrieves context info from server for a graph
//
// // use std::collections::HashMap;
// use js_sys::{Function, Object, Reflect, WebAssembly, Promise};
// use wasm_bindgen::prelude::*;
// use wasm_bindgen::JsCast;
// use wasm_bindgen_futures::{spawn_local, JsFuture, future_to_promise};
// use serde::{Deserialize, Serialize};
// use serde_json;
//
// use reqwest;
//
//
// #[wasm_bindgen]
// pub fn get_context(graph: String) -> Result<(), JsValue> {
//     let parsed_graph_steps = deserialize_graph(data).expect("Graph data could not be deserialized.");
//     let input_parsed = deserialize_inputs(inputs).expect("Graph data could not be deserialized.");
// }
