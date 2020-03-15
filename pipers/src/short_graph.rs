use std::collections::HashMap;
// use wasm_bindgen::prelude::*;
// use wasm_bindgen::JsCast;
use serde::{Deserialize, Serialize};
// use serde_json;

pub fn is_graph_input(index: u32) -> bool {
    index > 2999
}
pub fn is_graph_output(index: u32) -> bool {
    index > 3999
}

pub fn io_map_key(node_index: u32, io_index: u32) -> String {
    node_index.to_string() + "_" + &io_index.to_string()
}

pub trait CheckNodeType {
    fn is_graph_input(&self) -> bool;

    fn is_graph_output(&self) -> bool;
}

// pub trait NodeHandle<T> {
//     pub fn add(&self, node: T) {
//         self.insert(&node.index, node);
//     }
//
//     pub fn remove(&self, index: u32) {
//         self.remove(&index);
//     }
// }

#[derive(Clone)]
#[derive(Serialize, Deserialize, Debug)]
pub struct ShortGraphNode {
    pub index: u32,
    pub _id: String
}
impl CheckNodeType for ShortGraphNode {
    fn is_graph_input(&self) -> bool {
        is_graph_input(self.index)
    }

    fn is_graph_output(&self) -> bool {
        is_graph_output(self.index)
    }
}

#[derive(Clone)]
#[derive(Serialize, Deserialize, Debug)]
pub struct ShortGraphEdge {
    pub out_index: u32,
    pub out_output_index: u32,
    pub in_index: u32,
    pub in_input_index: u32,
}

impl ShortGraphEdge {
    pub fn out_key(&self) -> String {
        io_map_key(self.out_index, self.out_output_index)
    }

    pub fn in_key(&self) -> String {
        io_map_key(self.in_index, self.in_input_index)
    }
}

impl CheckNodeType for ShortGraphEdge {
    fn is_graph_input(&self) -> bool {
        is_graph_input(self.out_index)
    }

    fn is_graph_output(&self) -> bool {
        is_graph_output(self.in_index)
    }
}

pub type ShortGraphNodes = HashMap<u32, ShortGraphNode>;
pub type ShortGraphEdges = Vec<ShortGraphEdge>;

#[derive(Clone)]
#[derive(Serialize, Deserialize, Debug)]
pub struct ShortGraph {
    pub n: ShortGraphNodes,
    pub e: ShortGraphEdges,
}

// impl ShortGraphNodes {
//     pub fn new() -> ShortGraphNodes {
//         HashMap::new()
//     }
// }

impl ShortGraph {
    pub fn new() -> ShortGraph {
        ShortGraph {
            n: ShortGraphNodes::new(),
            e: ShortGraphEdges::new(),
        }
    }

    pub fn add_node(&mut self, index: u32, id: &String) {
        self.n.insert(index, ShortGraphNode{
            index: index,
            _id: id.to_owned(),
        });
    }
}

pub type Sources = HashMap<String, String>;

#[derive(Clone)]
#[derive(Serialize, Deserialize, Debug)]
pub struct FunctionIO {
    pub name: String,
    pub label: String,
}

pub type FunctionIOs = Vec<FunctionIO>;

#[derive(Clone)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Gapi {
    // pub constant: bool,
    pub inputs: FunctionIOs,
    pub outputs: FunctionIOs,
    // pub payable: bool,
    // pub state_mutability: ,
    pub type_choice: String,
}

#[derive(Clone)]
#[derive(Serialize, Deserialize, Debug)]
pub struct PFunction {
    pub name: String,
    pub signature: String,
    pub gapi: Gapi,
    // pub sources: Sources,
}

#[derive(Clone)]
#[derive(Serialize, Deserialize, Debug)]
pub struct PClass {
    pub name: String,
    pub url: String,
}

#[derive(Clone)]
#[derive(Serialize, Deserialize, Debug)]
pub struct NodeContext {
    pub _id: String,
    pub pclassid: String,
    pub pfunction: PFunction,
    pub pclass: PClass,
}

impl NodeContext {
    pub fn inputs(&self) -> &FunctionIOs {
        &self.pfunction.gapi.inputs
    }

    pub fn input(&self, index: u32) -> &FunctionIO {
        &self.inputs()[index as usize]
    }

    pub fn input_by_index(&self, index: u32) -> &FunctionIO {
        &self.inputs()[(index - 1) as usize]
    }

    pub fn outputs(&self) -> &FunctionIOs {
        &self.pfunction.gapi.outputs
    }

    pub fn output(&self, index: u32) -> &FunctionIO {
        &self.outputs()[index as usize]
    }

    pub fn output_by_index(&self, index: u32) -> &FunctionIO {
        &self.outputs()[(index - 1) as usize]
    }
}

pub type Context = Vec<NodeContext>;
// pub type ContextMap = HashMap<String, NodeContext>;

#[derive(Clone)]
#[derive(Serialize, Deserialize, Debug)]
pub struct ContextMap {
    // key is node._id
    pub map: HashMap<String, NodeContext>,
}

impl ContextMap {
    pub fn new() -> ContextMap {
        ContextMap {
            map: HashMap::new(),
        }
    }

    pub fn from_context(context: Context) -> ContextMap {
        let mut map = ContextMap::new();
        for node_context in context.iter() {
            // let cloned: NodeContext = *node_context.to_owned();
            // map.add(cloned);
            map.add(node_context);
        }
        map
    }

    pub fn add(&mut self, node_context: &NodeContext) {
        // let cloned: NodeContext = node_context.to_owned();
        self.map.insert(node_context._id.to_owned(), node_context.to_owned());
    }

    pub fn remove(&mut self, _id: &String) {
        self.map.remove(_id);
    }

    pub fn add_identity_input(&mut self, id: &String, input: FunctionIO) -> NodeContext {
        let node_context = build_input_context(&id, vec![input]);
        self.add(&node_context);
        node_context
    }

    pub fn add_identity_output(&mut self, id: &String, output: FunctionIO) -> NodeContext {
        let node_context = build_output_context(&id, vec![output]);
        self.add(&node_context);
        node_context
    }
}

#[derive(Clone)]
#[derive(Serialize, Deserialize, Debug)]
pub struct RichGraphNodePosition {
    pub x: u32,
    pub y: u32,
}

impl RichGraphNodePosition {
    pub fn new() -> RichGraphNodePosition {
        RichGraphNodePosition {
            x: 0,
            y: 0,
        }
    }
}

#[derive(Clone)]
#[derive(Serialize, Deserialize, Debug)]
pub struct NodeIoPair {
    pub node_index: u32,
    pub io_index: u32,
}

impl NodeIoPair {
    pub fn map_key(&self) -> String {
        io_map_key(self.node_index, self.io_index)
    }
}

// key = input index (io_index)
pub type RichGraphNodeIns = HashMap<u32, NodeIoPair>;

// key = output index (io_index)
pub type RichGraphNodeOuts = HashMap<u32, Vec<NodeIoPair>>;

// pub type RichGraphNodeIns = Vec<NodeIoPair>;
// pub type RichGraphNodeOuts = Vec<Vec<NodeIoPair>>;

#[derive(Clone)]
#[derive(Serialize, Deserialize, Debug)]
pub struct RichGraphNode {
    pub index: u32,
    pub _id: String,
    pub outs: RichGraphNodeOuts,
    pub ins: RichGraphNodeIns,
    pub position: RichGraphNodePosition,
    // pub edges: Vec<u32>
}

impl RichGraphNode {
    pub fn init(index: u32, _id: String) -> RichGraphNode{
        RichGraphNode {
            index: index,
            _id: _id.to_owned(),
            outs: RichGraphNodeOuts::new(),
            ins: RichGraphNodeIns::new(),
            position: RichGraphNodePosition::new(),
        }
    }
}
impl CheckNodeType for RichGraphNode {
    fn is_graph_input(&self) -> bool {
        is_graph_input(self.index)
    }

    fn is_graph_output(&self) -> bool {
        is_graph_output(self.index)
    }
}

pub type RichGraphNodes = HashMap<u32, RichGraphNode>;
pub type RichGraphEdges = ShortGraphEdges;

// impl NodeHandle for RichGraphNodes {
// impl RichGraphNodes {
//     fn add(&self, node: RichGraphNode) {
//         self.insert(&node.index, node);
//     }
//
//     fn remove(&self, index: u32) {
//         self.remove(&index);
//     }
// }

#[derive(Clone)]
#[derive(Serialize, Deserialize, Debug)]
pub struct RichGraph {
    pub n: RichGraphNodes,
    pub e: RichGraphEdges,
    // pub r: RichGraphRuntime,
}

impl RichGraph {
    pub fn new() -> RichGraph {
        RichGraph {
            n: RichGraphNodes::new(),
            e: RichGraphEdges::new(),
            // r: RichGraphRuntime::new(),
        }
    }
}

#[derive(Clone)]
#[derive(Serialize, Deserialize, Debug)]
pub struct RunnableShortGraph {
    pub steps: Vec<Vec<u32>>,
}

impl RunnableShortGraph {
    pub fn new() -> RunnableShortGraph {
        // let step: Vec<u32> = Vec::new();
        RunnableShortGraph {
            steps: Vec::new(),// vec![step],
        }
    }
    pub fn has_input(&self) -> bool {
        is_graph_input(self.steps[0][0])
    }

    pub fn has_output(&self) -> bool {
        is_graph_output(self.steps[self.steps.len() - 1][0])
    }
}

#[derive(Clone)]
#[derive(Serialize, Deserialize, Debug)]
pub struct RuntimeGraphs {
    pub short_graph: ShortGraph,
    pub interm_graph: ShortGraph,
    pub rich_graph: RichGraph,
    pub runnable_graph: RunnableShortGraph,
    pub context_map: ContextMap,
}

impl RuntimeGraphs {
    pub fn context_by_index(&self, index: u32) -> &NodeContext {
        let node = &self.rich_graph.n.get(&index).unwrap();
        self.context_map.map.get(&node._id).unwrap()
    }
}

const BASE_NODEID: &'static str = "5dbaa731f18ff7488e9b108b";
const BASE_PCLASSID: &'static str = "5dbaa731f18ff7488e9b108c";

pub fn build_input_context(id: &String, inputs: Vec<FunctionIO>)  -> NodeContext {
    build_io_context(id, inputs, Vec::new())
}

pub fn build_output_context(id: &String, outputs: Vec<FunctionIO>)  -> NodeContext {
    build_io_context(id, Vec::new(), outputs)
}

pub fn build_io_context(id: &String, inputs: Vec<FunctionIO>, outputs: Vec<FunctionIO>) -> NodeContext {
    NodeContext {
        _id: BASE_NODEID.to_owned() + (&id).clone(),
        pclassid: BASE_PCLASSID.to_owned(),
        pfunction: PFunction {
            name: String::from("identity"),
            signature: String::from("identity(x)"),
            gapi: Gapi {
                inputs: inputs,
                outputs: outputs,
                type_choice: String::from("fn"),
            }
        },
        pclass: PClass {
            name: "base".to_owned(),
            url: "".to_owned(),
        },
    }
}
