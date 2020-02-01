#![allow(unused_variables)]

use std::collections::HashMap;
use std::cmp;
use js_sys::{Function, Object, Reflect, WebAssembly, Promise};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{JsFuture, future_to_promise};
// use serde::{Deserialize, Serialize};
use serde_json;

use reqwest;

pub mod short_graph;
pub mod runtime_def;
// pub mod runtime;

use crate::short_graph::CheckNodeType;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

async fn get_module_buffer(url: &String) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let resp = reqwest::get(url)
        .await?
        .bytes()
        .await?;

    Ok(resp.to_vec())
}

async fn load_module_async(url: &String) -> Result<WebAssembly::Instance, JsValue> {
    let buffer = get_module_buffer(url).await.expect("get_module_buffer failed");

    let a = JsFuture::from(WebAssembly::instantiate_buffer(&buffer, &Object::new())).await?;

    console_log!("a {:?}", a);
    let b: WebAssembly::Instance = Reflect::get(&a, &"instance".into())?.dyn_into()?;

    Ok(b)
}

fn run_func(b: &WebAssembly::Instance, fdata: &runtime_def::FunctionInterface, inputs: &serde_json::Value) -> Result<serde_json::Value, JsValue> {
    let mut mutable_inputs = inputs.to_owned();
    let c = b.exports();
    console_log!("c: {:?}", c);
    console_log!("callF fdata: {:?}", fdata);

    // let sum = Reflect::get(c.as_ref(), &"add".into())?
    //     .dyn_into::<Function>()
    //     .expect("add export wasn't a function");
    //
    // console_log!("sum: {:?}", sum);
    //
    // let three = sum.call2(&JsValue::undefined(), &1.into(), &2.into())?;
    // console_log!("1 + 2 = {:?}", three);

    // let vec1 = vec![1, 2, 3];
    // let vec2 = vec![4, 5];
    // let jsvec1 = serde_wasm_bindgen::to_value(&vec1)?;
    // let jsvec2 = serde_wasm_bindgen::to_value(&vec2)?;
    //
    // console_log!("jsvec1 = {:?}", jsvec1);
    // console_log!("jsvec2 = {:?}", jsvec2);

    let fname = &fdata.name;
    let finstance = Reflect::get(c.as_ref(), &fname.into())?
        .dyn_into::<Function>()
        .expect(&format!("{} wasn't a function", fdata.name).to_string());

    console_log!("finstance = {:?}", finstance);
    let length: usize = fdata.inputs.len();
    console_log!("Inputs lenght: {}", length);
    let result = match &length {
        0 => finstance.call0(&JsValue::undefined()),
        1 => {
            let input = &mutable_inputs[&fdata.inputs[0].label];
            let inputjs: i32 = serde_json::from_value(input.to_owned())
                .expect("finstance.call1: serde_json::from_value failed");
            finstance.call1(&JsValue::undefined(), &inputjs.into())
        },
        2 => {
            let input1 = &mutable_inputs[&fdata.inputs[0].label];
            let input2 = &mutable_inputs[&fdata.inputs[1].label];
            let inputjs1: i32 = serde_json::from_value(input1.to_owned())
                .expect("finstance.call2: serde_json::from_value 1 failed");
            let inputjs2: i32 = serde_json::from_value(input2.to_owned())
                .expect("finstance.call2: serde_json::from_value 2 failed");
            console_log!("Inputs {:?}, {:?}", inputjs1, inputjs2);
            finstance.call2(&JsValue::undefined(), &inputjs1.into(), &inputjs2.into())
        },
        3 => {
            let input1 = &mutable_inputs[&fdata.inputs[0].label];
            let input2 = &mutable_inputs[&fdata.inputs[1].label];
            let input3 = &mutable_inputs[&fdata.inputs[2].label];
            let inputjs1: i32 = serde_json::from_value(input1.to_owned())
                .expect("finstance.call3: serde_json::from_value 1 failed");
            let inputjs2: i32 = serde_json::from_value(input2.to_owned())
                .expect("finstance.call3: serde_json::from_value 2 failed");
            let inputjs3: i32 = serde_json::from_value(input3.to_owned())
                .expect("finstance.call3: serde_json::from_value 3 failed");
            finstance.call3(&JsValue::undefined(), &inputjs1.into(), &inputjs2.into(), &inputjs3.into())
        },
        _ => panic!("Function has more than 3 args"),
    }?;

    console_log!("result = {:?}", result);
    // TODO how to do this dynamically? extend dyn_into?

    let out: i32 = result.into_serde().expect("result.into_serde failed");
    console_log!("out: {:?} = {:?}", &fdata.outputs[0].label, out);

    mutable_inputs[&fdata.outputs[0].label] = out.into();
    // mutable_inputs[&fdata.outputs[0].label] = 20.into();

    // let result = finstance.call2(&JsValue::undefined(), &jsvec1, &jsvec2)?;
    // console_log!("result = {:?}", result);
    //
    // let value: Vec<i32> = serde_wasm_bindgen::from_value(result)?;
    // console_log!("value = {:?}", value);

    // let result = finstance.call0(&JsValue::undefined())?;
    // console_log!("result = {:?}", result);

    // let value: String = serde_wasm_bindgen::from_value(result)?;
    // console_log!("value = {:?}", value);

    Ok(mutable_inputs)
}

fn deserialize_graph(data: String) -> serde_json::Result<runtime_def::GraphSteps> {
    let parsed: runtime_def::GraphSteps = serde_json::from_str(&data)?;
    // console_log!("parsed = {:?}", parsed);
    Ok(parsed)
}

fn deserialize_inputs(data: String) -> serde_json::Result<serde_json::Value> {
    let parsed: serde_json::Value = serde_json::from_str(&data)?;
    // console_log!("deserialize_inputs = {:?}", parsed);
    Ok(parsed)
}

fn run_graph(module_instances: &HashMap<String, WebAssembly::Instance>, steps: runtime_def::GraphSteps, inputs: serde_json::Value) -> Result<serde_json::Value, JsValue> {
    let mut mutable_inputs = inputs.to_owned();
    let mut step_start = 0;
    let mut step_stop = steps.len() - 1;

    if steps[0][0].index >= 3000 {
        step_start = 1;
    }
    if steps[step_stop][0].index >= 3000 {
        step_stop -= 1;
    }

    for level in &steps[step_start..=step_stop] {
        for step in level.iter() {
            console_log!("Step {}", step.name);
            let module_instance = match module_instances.get(&step.pclass.url) {
                Some(m) => m,
                None => {
                    return Err(JsValue::from(String::from("Module does not exist")))
                }
            };
            mutable_inputs = run_func(
                &module_instance,
                step,
                &mutable_inputs,
            ).expect("run_func failed");
        }
    }

    if step_stop == steps.len() - 1 {
        console_log!("---- outputs: []");
        return Ok(serde_json::json!([]));
    }

    let mut outputs = vec![];

    for out in steps[step_stop + 1].iter() {
        outputs.push(mutable_inputs[&out.inputs[0].label].to_owned());
    }

    // console_log!("---- outputs: {:?}", &outputs);

    let serde_output: serde_json::Value = outputs.into();
    Ok(serde_output)
}

#[wasm_bindgen]
pub fn run(data: String, inputs: String) -> Promise {
    let parsed_graph_steps = deserialize_graph(data).expect("Graph data could not be deserialized.");
    let input_parsed = deserialize_inputs(inputs).expect("Graph data could not be deserialized.");
    let mut module_instances = HashMap::new();

    future_to_promise(async move {
        for level in parsed_graph_steps.iter() {
            for step in level.iter() {
                if step.pclass.url == String::from("") {
                    break;
                }
                if module_instances.contains_key(&step.pclass.url) {
                    break;
                }
                let b = load_module_async(&step.pclass.url).await;
                let loaded_module = match b {
                    Ok(m) => m,
                    Err(m) => {
                        console_log!("Module loading error: {:?}", m);
                        // let b: WebAssembly::Instance = Reflect::get(&a, &"instance".into())?.dyn_into()?;

                        // console_log!("Module loading error: {}", m.description());
                        // console_log!("Module loading error: {}", m.source().unwrap());
                        // console_log!("Module loading error: {}", m.backtrace());
                        // &b.unwrap_throw();

                        return Err(JsValue::from(String::from("Module loading error")));
                    }
                };
                console_log!("Module : {:?}", &loaded_module);
                let c = loaded_module.exports();
                console_log!("Module exports : {:?}", Reflect::own_keys(c.as_ref()).unwrap());

                module_instances.insert(step.pclass.url.to_owned(), loaded_module);
            }
        }

        let outputs = run_graph(&module_instances, parsed_graph_steps, input_parsed).expect("run_graph failed");
        console_log!("---- outputs: {:?}", &outputs);
        console_log!("---- outputs: {:?}", &outputs.to_string());
        let result: String = outputs.to_string();

        Ok(JsValue::from(result))
    })
}

// !!!!!!!! runtime


fn deserialize_short_graph(data: String) -> serde_json::Result<short_graph::ShortGraph> {
    let parsed: short_graph::ShortGraph = serde_json::from_str(&data)?;
    // console_log!("parsed = {:?}", parsed);
    Ok(parsed)
}

fn deserialize_context(data: String) -> serde_json::Result<short_graph::Context> {
    let parsed: short_graph::Context = serde_json::from_str(&data)?;
    // console_log!("parsed = {:?}", parsed);
    Ok(parsed)
}

pub fn add_interm_graph_io(
    short_graph: &short_graph::ShortGraph,
    context_map: & short_graph::ContextMap,
) -> short_graph::ShortGraph {
    let mut interm_graph = short_graph.clone();
    let mut last_input_i = 3000;
    let mut last_output_i = 4000;

    // short_graph can contain graph input ids if the inputs serve multiple nodes
    while let Some(node) = short_graph.n.get(&last_input_i) {
        last_input_i += 1;
    }
    while let Some(node) = short_graph.n.get(&last_output_i) {
        last_output_i += 1;
    }

    let mut ins: HashMap<String, bool> = HashMap::new();
    let mut outs: HashMap<String, bool> = HashMap::new();
    for edge in short_graph.e.iter() {
        ins.insert(
            edge.out_index.to_string() + "_" + &edge.out_output_index.to_string(),
            true
        );
        outs.insert(
            edge.in_index.to_string() + "_" + &edge.in_input_index.to_string(),
            true
        );
    }

    for node in short_graph.n.values() {
        let node_context = &context_map.map.get(&node._id)
            .expect("add_interm_graph_io could not get node_context");
        // Go through all outputs of a node and see if they exist in graph.n
        // If not, add the corresponding edge; missing nodes are graph outputs
        // The rest of the info (e.g. graph.n) is added from the edges in build_interm_graph
        for (out_index, _) in node_context.outputs().iter().enumerate() {
            let used_index = out_index + 1;
            let key = node.index.to_string() + "_" + &used_index.to_string();
            if ins.contains_key(&key) {
                continue;
            }
            interm_graph.e.push(short_graph::ShortGraphEdge {
                out_index: node.index,
                out_output_index: used_index as u32,
                in_index: last_output_i,
                in_input_index: 1,
            });
            last_output_i += 1;
        }

        // Go through all inputs of a node and see if they exist in graph.n
        // If not, add the corresponding edge; missing nodes are graph inputs
        // The rest of the info (e.g. graph.n) is added from the edges in build_interm_graph
        for (in_index, _) in node_context.inputs().iter().enumerate() {
            let used_index = in_index + 1;
            let key = node.index.to_string() + "_" + &used_index.to_string();
            if outs.contains_key(&key) {
                continue;
            }
            interm_graph.e.push(short_graph::ShortGraphEdge {
                out_index: last_input_i,
                out_output_index: 1,
                in_index: node.index,
                in_input_index: used_index as u32,
            });
            last_input_i += 1;
        }
    }
    interm_graph
}

pub fn build_interm_graph(
    short_graph: &short_graph::ShortGraph,
    context_map: &mut short_graph::ContextMap,
) -> short_graph::ShortGraph {
    let mut interm_graph = add_interm_graph_io(&short_graph, &context_map);
    let cloned_graph = interm_graph.clone();

    // Add identity nodes - inputs
    for edge in cloned_graph.e.iter().filter(|edge| edge.is_graph_input()) {
        let target_node_id = &cloned_graph.n.get(&edge.in_index)
            .expect("target_node_id failed")
            ._id;
        let target_node = &context_map.map.get(target_node_id)
            .expect("target_node failed");
        let target_input = target_node.input_by_index(edge.in_input_index).to_owned();

        let input = context_map.add_identity_input(&edge.out_index.to_string(), target_input);
        interm_graph.add_node(edge.out_index, &input._id);
    }

    // Add identity nodes - outputs
    for edge in cloned_graph.e.iter().filter(|edge| edge.is_graph_output()) {
        let target_node_id = &cloned_graph.n.get(&edge.out_index)
            .expect("target_node_id failed")
            ._id;
        let target_node = &context_map.map.get(target_node_id)
            .expect("target_node_id failed");
        let target_input = target_node.input_by_index(edge.out_output_index).to_owned();

        let output = context_map.add_identity_output(&edge.in_index.to_string(), target_input);
        interm_graph.add_node(edge.in_index, &output._id);
    }
    interm_graph
}

fn enrich_graph_edges(
    interm_graph: &short_graph::ShortGraph,
    rich_graph: &mut short_graph::RichGraph,
) {
    // Add outs and ins for each graph node, from edge info
    // E.g. {out_index: 3001, out_output_index: 1, in_index: 1, in_input_index: 2}
    // --------   --1--2--
    // | 3001 |   |  1   |     output 1 from node 3001 -> input 2 from node 1
    // ---1----   --------
    // node 3001 will have {1, 2} added in outs.1
    // node 1 will have {3001, 1} added in ins.2
    for edge in interm_graph.e.iter() {
        let outnode = rich_graph.n.get_mut(&edge.out_index).expect("enrich_graph_edges outnode failed");
        let output = short_graph::NodeIoPair {
            node_index: edge.in_index,
            io_index: edge.in_input_index,
        };
        match outnode.outs.get_mut(&edge.out_output_index) {
            Some(outs) => outs.push(output),
            None => {
                let mut outputs = Vec::new();
                outputs.push(output);
                outnode.outs.insert(edge.out_output_index, outputs);
            }
        };

        let innode = rich_graph.n.get_mut(&edge.in_index).expect("enrich_graph_edges innode failed");
        let input = short_graph::NodeIoPair {
            node_index: edge.out_index,
            io_index: edge.out_output_index,
        };
        match innode.ins.get_mut(&edge.in_input_index) {
            Some(_) => panic!("Input has multiple linked outputs!"),
            None => {
                innode.ins.insert(edge.in_input_index, input);
            }
        };
    }
}

fn input_nodes_are_visited(unvisited_node: &short_graph::RichGraphNode, visited: &Vec<u32>) -> bool {
    for external_input_i in unvisited_node.ins.keys() {
        let external_input = &unvisited_node.ins.get(external_input_i).unwrap();
        if !visited.contains(&external_input.node_index) {
            return false;
        }
    }
    true
}

// fn inputNodesAreDefined(unvisited_node: u32, rich_graph: &Vec<u32>, level: u32) -> bool {
//
// }

fn enrich_visitor(
    context_map: &short_graph::ContextMap,
    unvisited_node: &mut short_graph::RichGraphNode,
    current_point: &mut (u32, u32),
    level: u32,
    row: u32,
) {
    current_point.1 = level;
    if row == 0 {
        current_point.0 = 0;
    }

    unvisited_node.position.x = current_point.0;
    unvisited_node.position.y = current_point.1;

    let node_context = context_map.map.get(&unvisited_node._id).unwrap();

    let max = cmp::max(
        node_context.inputs().len(),
        node_context.outputs().len(),
    );
    current_point.0 = current_point.0 + max as u32;
}

fn difference_unvisited(unvisited: &mut Vec<u32>, visited_now: &Vec<u32>) {
    for visited in visited_now.iter() {
        while let Some(index) = unvisited.iter().position(|n| n == visited) {
            unvisited.remove(index);
        }
    }
}

fn enrich_graph_position_nodes(
    context_map: &short_graph::ContextMap,
    mut rich_graph: &mut short_graph::RichGraph,
    mut unvisited: &mut Vec<u32>,
    mut visited: &mut Vec<u32>,
    mut current_point: &mut (u32, u32),
    level: u32,
) {
    if unvisited.len() == 0 {
        return;
    }

    let mut row: u32 = 0;
    let mut visited_now: Vec<u32> = vec![];
    let mut only_outputs = true;

    if unvisited.iter().find(|n| !short_graph::is_graph_output(*n.to_owned())) != None {
        only_outputs = false;
    }

    for unvisited_node_i in unvisited.iter() {
        let mut unvisited_node = rich_graph.n.get_mut(&unvisited_node_i).unwrap();
        let are_visited: bool = input_nodes_are_visited(&unvisited_node, &visited);
        // let are_defined: bool = inputNodesAreDefined(unvisited_node, &rich_graph.r, level);
        let are_defined = false;
        let not_input_level: bool = level > 0;
        let is_io: bool = short_graph::is_graph_input(*unvisited_node_i);
        let is_not_output: bool = !short_graph::is_graph_output(*unvisited_node_i);
        let doit: bool = (are_visited || are_defined) && (not_input_level || is_io);

        // console_log!(
        //     "unvisited_node: {:?} ; are_visited: {:?} ; are_defined: {:?} ; not_input_level: {:?} ; is_io: {:?} ; doit: {:?} ;",
        //     unvisited_node, are_visited, are_defined, not_input_level, is_io, doit
        // );

        if doit {
            enrich_visitor(context_map, &mut unvisited_node, &mut current_point, level, row);

            row += 1;
            if only_outputs || is_not_output {
                visited_now.push(*unvisited_node_i);
            }
        }
    }

    difference_unvisited(&mut unvisited, &visited_now);
    visited.extend_from_slice(&visited_now.as_slice());

    enrich_graph_position_nodes(
        &context_map,
        &mut rich_graph,
        &mut unvisited,
        &mut visited,
        &mut current_point,
        level + 1,
    );
}

pub fn enrich_graph(
    interm_graph: &short_graph::ShortGraph,
    context_map: &mut short_graph::ContextMap,
) -> short_graph::RichGraph {

    let mut rich_graph = short_graph::RichGraph::new();

    // Initialize rich graph
    for (key, value) in &interm_graph.n {
        let node = interm_graph.n.get(&key).expect("enrich_graph get node failed");
        rich_graph.n.insert(
            key.to_owned(),
            short_graph::RichGraphNode::init(node.index, node._id.to_owned())
        );
    }
    rich_graph.e = interm_graph.e.clone();

    enrich_graph_edges(&interm_graph, &mut rich_graph);

    let mut visited: Vec<u32> = Vec::new();
    visited.push(0);
    let mut unvisited: Vec<u32> = rich_graph.n.keys().map(|key| *key).collect();
    unvisited.sort();
    let mut current_point: (u32, u32) = (0, 1);
    enrich_graph_position_nodes(
        &context_map,
        &mut rich_graph,
        &mut unvisited,
        &mut visited,
        &mut current_point,
        0,
    );

    rich_graph
}

fn get_runnable_graph(rich_graph: &short_graph::RichGraph) -> short_graph::RunnableGraph {
    let mut runnable = short_graph::RunnableGraph::new();
    let mut keys: Vec<u32> = rich_graph.n.keys().map(|key| *key).collect();
    keys.sort();

    for key in keys.iter() {
        let node = rich_graph.n.get(key).unwrap();
        let len = runnable.steps.len();
        let y = node.position.y as usize;

        for number in len..=y {
            runnable.steps.push(Vec::new());
        }

        runnable.steps[node.position.y as usize].push(node.index);
    }
    runnable
}

#[wasm_bindgen]
pub fn runtime(graph: String, context: String) -> Result<JsValue, JsValue> {
    let short_graph = deserialize_short_graph(graph).expect("Graph data could not be deserialized.");
    let context_parsed = deserialize_context(context).expect("Graph data could not be deserialized.");

    let mut context_map = short_graph::ContextMap::from_context(context_parsed);
    // console_log!("---- context_map {:?}", context_map);
    let interm_graph = build_interm_graph(&short_graph, &mut context_map);
    console_log!("interm_graph: {:?}", interm_graph);
    let rich_graph = enrich_graph(&interm_graph, &mut context_map);

    console_log!("---- rich_graph = {:?}", rich_graph);

    let runnable_graph = get_runnable_graph(&rich_graph);

    console_log!("---- runnable_graph = {:?}", runnable_graph);

    let result: String = serde_json::to_string(&rich_graph).expect("Rich graph could not be serialized");

    Ok(JsValue::from(result))
}
