import CONFIG from './config.js';

export const getUrl = (name) => `${CONFIG.WASM_SERVER.URL}/${name}/pkg/${name}_bg.wasm`;

export const graphSteps_i32 = [
  [
    {
      index: 1,
      name: 'sum_i32',
      inputs: [
        {
          name: 'a',
          ttype: 'i32',
        },
        {
          name: 'b',
          ttype: 'i32',
        }
      ],
      outputs: [
        {
          name: 'c',
          ttype: 'i32',
        }
      ],
      pclass: {
        name: 'sums',
        url: getUrl('sums'),
      }
    },
    {
      index: 2,
      name: 'sub_i32',
      inputs: [
        {
          name: 'c',
          ttype: 'i32',
        },
        {
          name: 'd',
          ttype: 'i32',
        }
      ],
      outputs: [
        {
          name: 'e',
          ttype: 'i32',
        }
      ],
      pclass: {
        name: 'subs',
        url: getUrl('subs'),
      }
    }
  ],
  [
    {
      index: 4001,
      name: 'outs',
      inputs: [
        {
          name: 'e',
          ttype: 'i32',
        }
      ],
      outputs: [],
      pclass: {
        name: '',
        url: '',
      }
    }
  ]
]

export const graphSteps_u32 = [
  [
    {
      index: 1,
      name: 'sum_u32',
      inputs: [
        {
          name: 'a',
          ttype: 'u32',
        },
        {
          name: 'b',
          ttype: 'u32',
        }
      ],
      outputs: [
        {
          name: 'c',
          ttype: 'u32',
        }
      ],
      pclass: {
        name: 'sums',
        url: getUrl('sums'),
      }
    },
    {
      index: 2,
      name: 'sub_u32',
      inputs: [
        {
          name: 'c',
          ttype: 'u32',
        },
        {
          name: 'd',
          ttype: 'u32',
        }
      ],
      outputs: [
        {
          name: 'e',
          ttype: 'u32',
        }
      ],
      pclass: {
        name: 'subs',
        url: getUrl('subs'),
      }
    }
  ],
  [
    {
      index: 4001,
      name: 'outs',
      inputs: [
        {
          name: 'e',
          ttype: 'i32',
        }
      ],
      outputs: [],
      pclass: {
        name: '',
        url: '',
      }
    }
  ]
]

export const graphSteps_i64 = [
  [
    {
      index: 1,
      name: 'sum_i64',
      inputs: [
        {
          name: 'a',
          ttype: 'i64',
        },
        {
          name: 'b',
          ttype: 'i64',
        }
      ],
      outputs: [
        {
          name: 'c',
          ttype: 'i64',
        }
      ],
      pclass: {
        name: 'sums',
        url: getUrl('sums'),
      }
    },
    {
      index: 2,
      name: 'sub_i64',
      inputs: [
        {
          name: 'c',
          ttype: 'i64',
        },
        {
          name: 'd',
          ttype: 'i64',
        }
      ],
      outputs: [
        {
          name: 'e',
          ttype: 'i64',
        }
      ],
      pclass: {
        name: 'subs',
        url: getUrl('subs'),
      }
    }
  ],
  [
    {
      index: 4001,
      name: 'outs',
      inputs: [
        {
          name: 'e',
          ttype: 'i32',
        }
      ],
      outputs: [],
      pclass: {
        name: '',
        url: '',
      }
    }
  ]
]
