// // builds runtime graph from graph + context
// #![allow(unused_variables)]
//
// // use std::collections::HashMap;
// // use js_sys::{Function, Object, Reflect, WebAssembly, Promise};
// use wasm_bindgen::prelude::*;
// use wasm_bindgen::JsCast;
// // use wasm_bindgen_futures::{spawn_local, JsFuture, future_to_promise};
// // use serde::{Deserialize, Serialize};
// use serde_json;
//
// // mod crate::short_graph;
// // TODO remove:
// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(js_namespace = console)]
//     fn log(a: &str);
// }
//
// macro_rules! console_log {
//     ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
// }
//
// fn deserialize_graph(data: String) -> serde_json::Result<crate::short_graph::ShortGraph> {
//     let parsed: crate::short_graph::ShortGraph = serde_json::from_str(&data)?;
//     console_log!("parsed = {:?}", parsed);
//     Ok(parsed)
// }
//
// fn deserialize_context(data: String) -> serde_json::Result<crate::short_graph::Context> {
//     let parsed: crate::short_graph::Context = serde_json::from_str(&data)?;
//     console_log!("parsed = {:?}", parsed);
//     Ok(parsed)
// }
//
// #[wasm_bindgen]
// pub fn runtime(graph: String, context: String) -> Result<(), JsValue> {
//     console_log!("graph = {:?}", graph);
//     console_log!("context = {:?}", context);
//
//     // let parsed_graph_steps = deserialize_graph(graph).expect("Graph data could not be deserialized.");
//     // let context_parsed = deserialize_context(context).expect("Graph data could not be deserialized.");
//
//     // Ok(JsValue::from(String::from("hello")))
//     Ok(())
// }
