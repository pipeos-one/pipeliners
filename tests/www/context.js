import CONFIG from './config.js';

export const sgraph_i32 = {
  n: {1: {index: 1, _id: "5dbbf356f18ff7488e9b1097"}, 2: {index: 2, _id: "5dbbf356f18ff7488e9b1098"}},
  e: [[1, 1, 2, 1]]
}

const getUrl = (name) => `${CONFIG.WASM_SERVER.URL}/${name}/pkg/${name}_bg.wasm`;

export const libs_data = [
  {
    name: "wasm_sums",
    address: getUrl('sums'),
    abi: [
      {"constant":false,"inputs":[{"name":"a","type":"int8"},{"name":"b","type":"int8"}],"name":"sum_i8","outputs":[{"name":"sum_i8","type":"int8"}],"payable":false,"stateMutability":"nonpayable","type":"function"},
      {"constant":false,"inputs":[{"name":"a","type":"int16"},{"name":"b","type":"int16"}],"name":"sum_i16","outputs":[{"name":"sum_i16","type":"int16"}],"payable":false,"stateMutability":"nonpayable","type":"function"},
      {"constant":false,"inputs":[{"name":"a","type":"int32"},{"name":"b","type":"int32"}],"name":"sum_i32","outputs":[{"name":"sum_i32","type":"int32"}],"payable":false,"stateMutability":"nonpayable","type":"function"}
    ],
  },
  {
    name: "wasm_subs",
    address: getUrl('subs'),
    abi: [
      {"constant":false,"inputs":[{"name":"a","type":"int8"},{"name":"b","type":"int8"}],"name":"sub_i8","outputs":[{"name":"sub_i8","type":"int8"}],"payable":false,"stateMutability":"nonpayable","type":"function"},
      {"constant":false,"inputs":[{"name":"a","type":"int16"},{"name":"b","type":"int16"}],"name":"sub_i16","outputs":[{"name":"sub_i16","type":"int16"}],"payable":false,"stateMutability":"nonpayable","type":"function"},
      {"constant":false,"inputs":[{"name":"a","type":"int32"},{"name":"b","type":"int32"}],"name":"sub_i32","outputs":[{"name":"sub_i32","type":"int32"}],"payable":false,"stateMutability":"nonpayable","type":"function"}
    ],
  },
]

const all_context = [];


export const context_i32 = [
  {
    _id: "5dbbf356f18ff7488e9b1097",
    pclassid: "5dbaa731f18ff7488e9b108b",
    pfunction: {
      name: 'sum_i32',
      signature: "sum_i32(i32,i32)",
      gapi: {
        type_choice: "fn",
        inputs: [
          {
            label: 'a',
            name: 'i32',
          },
          {
            label: 'b',
            name: 'i32',
          }
        ],
        outputs: [
          {
            label: 'c',
            name: 'i32',
          }
        ]
      },
      sources: {},
    },
    pclass: {
      name: 'sums',
      url: getUrl('sums'),
    }
  },
  {
    _id: "5dbbf356f18ff7488e9b1098",
    pclassid: "5dbaa731f18ff7488e9b108b",
    pfunction: {
      name: 'sub_i32',
      signature: "sub_i32(i32,i32)",
      gapi: {
        type_choice: "fn",
        inputs: [
          {
            label: 'a',
            name: 'i32',
          },
          {
            label: 'b',
            name: 'i32',
          }
        ],
        outputs: [
          {
            label: 'c',
            name: 'i32',
          }
        ]
      },
      sources: {},
    },
    pclass: {
      name: 'subs',
      url: getUrl('subs'),
    }
  }
]
