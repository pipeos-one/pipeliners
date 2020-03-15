import * as pipewasm from "pipewasm";
// import * as graphExamples from './graph_examples';
import { getUrl } from './graph_examples';
import * as graphContext from './context';

import CONFIG from './config.js';

console.log(JSON.stringify(graphContext.libs_data));

console.log('pipewasm', pipewasm);

async function doit(graphInput, contextInput, inputInput) {
  let result = await pipewasm.execute(graphInput, contextInput, inputInput);
  console.log('---- result', JSON.parse(result));
}

document.getElementById("executeGraph").onclick = () => {
  const {graphInput, contextInput, inputInput} = prepData(
    document.getElementById("graphInput").value,
    document.getElementById("contextInput").value,
    document.getElementById("inputInput").value
  )
  doit(graphInput, contextInput, inputInput);
}

function prepData(graphInput, contextInput, inputInput) {
  graphInput = JSON.parse(graphInput);
  delete graphInput.r;
  Object.keys(graphInput.n).forEach(node_i => {
    graphInput.n[node_i] = {
      index: graphInput.n[node_i].i,
      _id: graphInput.n[node_i].id
    }
  })

  graphInput = JSON.stringify(graphInput);

  const ctx = JSON.parse(contextInput);
  contextInput = Object.values(ctx).map(item => {
    return {
      _id: item._id,
      pclassid: item.pclassid,
      pfunction: {
        name: item.pfunction.gapi.name,
        signature: item.pfunction.signature,
        gapi: {
          inputs: item.pfunction.gapi.inputs.map(inp => {return {name: inp.type, label: inp.name}}),
          outputs: item.pfunction.gapi.outputs.map(inp => {return {name: inp.type, label: inp.name}}),
          type_choice: 'fn'
        },
      },
      pclass: {
        name: item.pclass.name,
        url: item.pclass.deployment,
      }
    }
  });
  contextInput = JSON.stringify(contextInput);

  // console.log(graphInput);
  // console.log(contextInput);
  // console.log(inputInput);
  return {graphInput, contextInput, inputInput};
}

// async function doit(graphInput, contextInput, inputInput) {
//   let result;
//   // result = await pipewasm.run(JSON.stringify(graphExamples.graphSteps_i32), JSON.stringify(input1))
//   //
//   // console.log('---- result', result);
//
//   // result = await pipewasm.run(JSON.stringify(graphExamples.graphSteps_u32), JSON.stringify(input2))
//   //
//   // console.log('---- result', result, JSON.parse(result));
//   // console.log(graphContext.sgraph_i32, graphContext.context_i32);
//
//   // result = await pipewasm.runtime(
//   //   JSON.stringify(graphContext.sgraph_i32),
//   //   JSON.stringify(graphContext.context_i32),
//   // );
//   //
//   // console.log('---- result', JSON.parse(result));
//
//   // result = await pipewasm.execute(
//   //   JSON.stringify(graphContext.sgraph_i32),
//   //   JSON.stringify(graphContext.context_i32),
//   //   JSON.stringify([210, 7, 3]),
//   // );
//
//   result = await pipewasm.execute(graphInput, contextInput, inputInput);
//
//   console.log('---- result', JSON.parse(result));
// }



// result = pipewasm.run(JSON.stringify(graphExamples.graphSteps_i64), JSON.stringify(input))


// pipewasm.then(async pipe => {
//   console.log('pipe', pipe)
//   // const nodes = await buildSource(context, graph);
//   // const steps = prepNodes(nodes);
//   // console.log('steps', steps);
//   // // console.log('ss', JSON.stringify(steps.slice(1, 3)));
//   // const result = pipe.run(JSON.stringify(steps), JSON.stringify(input));
//   const result = pipe.run(JSON.stringify(graphSteps), JSON.stringify(input))
//   console.log('result', result);
// }).catch(console.log);

function prepNodes(nodes) {
  return nodes.map(level => level.map(node => {
    return {
      pclass: {name: 'i32lib', path: './native/libs/i32lib/target/wasm32-unknown-unknown/debug/'},
      i: node.i,
      name: node.record.pfunction.gapi.name,
      inputs: (node.inputs || []).map(inp => {
        return {
          name: inp.name,
          ttype: inp.type,
        }
      }),
      outputs: (node.record.pfunction.gapi.outputs_idx || []).map(inp => {
        return {
          name: inp.name,
          ttype: inp.type,
        }
      }),
    }
  }));
}

// doit(
//   JSON.stringify(graphContext.sgraph_i32),
//   JSON.stringify(graphContext.context_i32),
//   JSON.stringify([210, 7, 3]),
// );

document.getElementById("graphInput").value = JSON.stringify(
  {"n":{"101":{"i":101,"id":"wasm_sums_0_sum_i32_2"},"102":{"i":102,"id":"wasm_subs_1_sub_i32_2"}},"e":[[101,1,102,1]],"r":[]}
);
document.getElementById("contextInput").value = JSON.stringify(
  {"wasm_sums_0_sum_i32_2":{"_id":"wasm_sums_0_sum_i32_2","pclassid":"wasm_sums_0","pfunction":{"signature":"","gapi":{"constant":false,"inputs":[{"name":"a","type":"int32"},{"name":"b","type":"int32"}],"name":"sum_i32","outputs":[{"name":"sum_i32","type":"int32"}],"payable":false,"stateMutability":"nonpayable","type":"function"},"graph":{},"sources":{}},"timestamp":"2019-03-22T14:38:36.112Z","pclass":{"_id":"wasm_sums_0","name":"wasm_sums","type":"sol","deployment": getUrl("sums")}},"wasm_subs_1_sub_i32_2":{"_id":"wasm_subs_1_sub_i32_2","pclassid":"wasm_subs_1","pfunction":{"signature":"","gapi":{"constant":false,"inputs":[{"name":"a","type":"int32"},{"name":"b","type":"int32"}],"name":"sub_i32","outputs":[{"name":"sub_i32","type":"int32"}],"payable":false,"stateMutability":"nonpayable","type":"function"},"graph":{},"sources":{}},"timestamp":"2019-03-22T14:38:36.112Z","pclass":{"_id":"wasm_subs_1","name":"wasm_subs","type":"sol","deployment":getUrl("subs")}}}
)
document.getElementById("inputInput").value = JSON.stringify([210, 7, 3]);
