import { coins } from "@cosmjs/proto-signing"

// This file holds sample data sets to be iterated over to generate all the scenarios needed to support

const to_binary = (v: any) => Buffer.from(JSON.stringify(v)).toString('base64')

// {
//   "actions": [
//     {
//       "msg": {
//         "wasm": {
//           "execute": {
//             "contract_addr": versions.manager.contract_addr,
//             "msg": Buffer.from(JSON.stringify({ "tick": {} })).toString('base64'),
//             "funds": []
//           }
//         }
//       },
//       "gas_limit": 75000
//     }
//   ],
//   "boundary": null,
//   "cw20": null,
//   "interval": {
//     "block": 1
//   },
//   "stop_on_fail": true,
//   "queries": null,
//   "transforms": null
// }

// Not valid itself, but easy to swap for real data
export const baseTask = {
  "actions": [],
  "boundary": null,
  "cw20": null,
  "interval": "Once",
  "stop_on_fail": true,
  "queries": null,
  "transforms": null,
}


export const intervals = [
  'once',
  'immediate',
  { block: 1 },
  { block: 2 },
  { block: 5 },
  { cron: '* * * * * *' },
  { cron: '1 * * * * *' },
  { cron: '* 0 * * * *' },
]

// Compute "now" before assigning the values here
const nanos = 1_000_000
const minute = 60 * 1000
const fiveminute = 5 * 60 * 1000
export const boundaries = (currentHeight: number) => [
  null,
  {
    height: {
      start: null,
      end: null,
    }
  },
  {
    height: {
      start: `${currentHeight}`,
      end: null,
    }
  },
  {
    height: {
      start: `${currentHeight}`,
      end: `${currentHeight + 100}`,
    }
  },
  {
    height: {
      start: null,
      end: `${currentHeight + 100}`,
    }
  },
  {
    time: {
      start: null,
      end: null,
    }
  },
  {
    time: {
      start: `${+new Date() * nanos}`,
      end: null,
    }
  },
  {
    time: {
      start: `${+new Date() * nanos}`,
      end: `${(+new Date() + fiveminute) * nanos}`,
    }
  },
  {
    time: {
      start: null,
      end: `${(+new Date() + fiveminute) * nanos}`,
    }
  },
]

export const actions = (options: any) => [
  {
    "msg": {
      "bank": {
        "send": {
          "to_address": options.address,
          "amount": coins(options.amount, options.denom)
        }
      }
    },
    "gas_limit": 75000
  },
  {
    "msg": {
      "wasm": {
        "execute": {
          "contract_addr": options.contract_addr,
          "msg": to_binary(options.action_msg || { "tick": {} }),
          "funds": []
        }
      }
    },
    "gas_limit": 75000
  },
  // TODO: Add more examples
]

// TWO Types of helpers here:
// 1. Query msg formatters
// 2. Option generators

// Option Generators
export const comparators = ['eq', 'ne', 'gt', 'gte', 'lt', 'lte']
export const status = ['open', 'rejected', 'passed', 'executed', 'closed', 'executionfailed']
export const valueOrdering = ['equal', 'notequal', 'unitabove', 'unitaboveequal', 'unitbelow', 'unitbelowequal']

export const comparatorToValueOrder = (v: string) => valueOrdering[comparators.indexOf(v)]

// Query Formatters

export const balances = {
  // {
  //   "get_balance": {
  //     "address": "juno1...",
  //     "denom": "ujunox"
  //   }
  // }
  getBalance({ address, denom }: { address: string, denom: string }) {
    return { get_balance: { address, denom } }
  },
  // {
  //   "get_cw20_balance": {
  //     "cw20_contract": "juno1fzqhwqczcz7z6s7ca6hgk9rpqv8qp6kq3j7uejf52efc03lgxn7sa8gslp",
  //     "address": "juno1..."
  //   }
  // }
  getCw20Balance({ cw20_contract, address }: { cw20_contract: string, address: string }) {
    return { get_cw20_balance: { cw20_contract, address } }
  },
  // {
  //   "has_balance_comparator": {
  //     "address": "juno1...",
  //       "comparator": "gte",
  //         "required_balance": {
  //       "native": [
  //         {
  //           "amount": "1000000",
  //           "denom": "ujunox"
  //         }
  //       ]
  //     }
  //   }
  // }
  getBalanceComparator({ address, required_balance, comparator }: { address: string, required_balance: any, comparator: string }) {
    return { has_balance_comparator: { address, required_balance, comparator } }
  },
}

// NOTE: In DAODAO V2, root DAO address isn't the right one, thats like a factory. 
// Must use the following to get the proposals module first:
// {
//   "proposal_modules": {}
// }
// 
// Response address will be:
// res[0].address

export const dao = {
  // {
  //   "proposal_status_matches": {
  //     "dao_address": "juno16skk5s8qcn4xmpq4j7e8r78zru5n2uvrsjdh7h74swpnzwnlagjqwues9x",
  //     "proposal_id": 1,
  //     "status": "rejected"
  //   }
  // }
  proposalStatusMatches({ dao_address, proposal_id, status }: { dao_address: string, proposal_id: number, status: string }) {
    return { proposal_status_matches: { dao_address, proposal_id, status } }
  },
  // {
  //   "has_passed_proposals": {
  //     "dao_address": "juno16skk5s8qcn4xmpq4j7e8r78zru5n2uvrsjdh7h74swpnzwnlagjqwues9x"
  //   }
  // }
  hasPassedProposals({ dao_address }: { dao_address: string }) {
    return { has_passed_proposals: { dao_address } }
  },
  // {
  //   "has_passed_proposal_with_migration": {
  //     "dao_address": "juno16skk5s8qcn4xmpq4j7e8r78zru5n2uvrsjdh7h74swpnzwnlagjqwues9x"
  //   }
  // }
  hasPassedProposalsWithMigration({ dao_address }: { dao_address: string }) {
    return { has_passed_proposals_with_migration: { dao_address } }
  },
  // {
  //   "has_proposals_gt_id": {
  //     "dao_address": "juno16skk5s8qcn4xmpq4j7e8r78zru5n2uvrsjdh7h74swpnzwnlagjqwues9x",
  //     "value": 1
  //   }
  // }
  hasProposalsGtId({ dao_address, value }: { dao_address: string, value: number }) {
    return { has_proposals_gt_id: { dao_address, value } }
  },
}

export const generic = {
  // {
  //   "generic_query": {
  //     "contract_addr": "juno1n88grnt3ajesp3x2wgx7535qlcw68720jrshh5gwz2sjzzq5gzksumx4n0",
  //     "msg": "ewogICAgICAgICJnZXRfYmFsYW5jZSI6IHsKICAgICAgICAgICJhZGRyZXNzIjogImp1bm8xcWxtd2prZzd1dTRhd2FqdzVhdW5jdGpkY2U5cTY1N2owcnJkcHkiLAogICAgICAgICAgImRlbm9tIjogInVqdW5veCIKICAgICAgICB9CiAgICAgIH0=",
  //     "path_to_value": [
  //       {
  //         "key": "result"
  //       }
  //     ],
  //     "ordering": "equal",
  //     "value": "eyJkZW5vbSI6InVqdW5veCIsImFtb3VudCI6IjQ4ODc5MjgxMzgifQ=="
  //   }
  // }
  genericQuery({ contract_addr, msg, path_to_value, ordering, value }: { contract_addr: string, msg: any, path_to_value: any, ordering: string, value: string }) {
    return { generic_query: { contract_addr, msg, path_to_value, ordering, value } }
  },
}

export const nft = {
  // {
  //   "owner_of_nft": {
  //     "address": "stars1...",
  //     "nft_address": "stars1...",
  //     "token_id": "4079"
  //   }
  // }
  ownerOfNft({ address, nft_address, token_id }: { address: string, nft_address: string, token_id: string }) {
    return { owner_of_nft: { address, nft_address, token_id } }
  },
  // {
  //   "addr_has_nft": {
  //     "address": "stars1...",
  //     "nft_address": "stars1..."
  //   }
  // }
  addrHasNft({ address, nft_address }: { address: string, nft_address: string }) {
    return { addr_has_nft: { address, nft_address } }
  },
}

export const queries = {
  balances,
  dao,
  generic,
  nft,
}

export const supportedModuleKeys = () => {
  let keys = []

  Object.keys(queries).forEach(k => {
    Object.keys(queries[k]).forEach(d => {
      keys.push(d)
    })
  })

  return keys
}

// The totes magix AMIRIGHT
export const getQueryMsgByTypes = (contract_addr: string, type: string, method: string, args: any, check_result: boolean) => {
  // {
  //   "contract_addr": "juno1...",
  //   "msg": {
  //     "has_balance_comparator": {
  //       "address": "juno1...",
  //         "comparator": "gte",
  //           "required_balance": {
  //         "native": [
  //           {
  //             "amount": "1000000",
  //             "denom": "ujunox"
  //           }
  //         ]
  //       }
  //     }
  //   },
  //   check_result: true,
  // }
  return {
    msg: to_binary(queries[type][method](args)),
    contract_addr: contract_addr,
    check_result,
  }
}

export const parsePathToArray = (p: string): any => {
  return p.split('.').map(i => {
    // check if integer or not for index-based checks
    return /^[0-9]{0,4}$/.test(i) ? { index: parseInt(i, 10) } : { key: i }
  })
}

export const getTransform = (
  query_idx: number,
  action_idx: number,
  query_path: string,
  action_path: string,
) => {
  // {
  //   "query_idx": 1,
  //   "action_idx": 0,
  //   "query_response_path": [
  //     {
  //       "key": "transfer"
  //     },
  //     {
  //       "key": "amount"
  //     }
  //   ],
  //   "action_path": [
  //     {
  //       "key": "admin"
  //     }
  //   ]
  // },
  return {
    query_idx,
    action_idx,
    query_response_path: parsePathToArray(query_path),
    action_path: parsePathToArray(action_path),
  }
}

// helpful transform examples
export const transforms = [
  getTransform(0, 0, 'transfer.amount', 'admin'),
]

// Generate a large set of query tasks
// options = { modBalancesAddr, currentHeight, address, cw20_contract, amount, denom, comparator }
export const getEventedTasks = (options: any) => {
  const eventedTasks = []
  console.log('options', options);

  // // BATCH TEST samples
  // eventedTasks.push({
  //   ...baseTask,
  //   interval: intervals[1], // immediate
  //   // interval: intervals[5], // 5 blocks
  //   boundary: boundaries(options.currentHeight)[0], // null
  //   actions: [actions(options)[0]], // bank send
  //   queries: [
  //     getQueryMsgByTypes(options.modBalancesAddr, 'balances', 'getBalance', { address: options.address, denom: options.denom }, true),
  //   ],
  //   transforms: [],
  // })
  // eventedTasks.push({
  //   ...baseTask,
  //   interval: intervals[1], // immediate
  //   // interval: intervals[6], // 5 blocks
  //   boundary: boundaries(options.currentHeight)[1], // null
  //   actions: [actions(options)[0]], // bank send
  //   queries: [
  //     getQueryMsgByTypes(options.modBalancesAddr, 'balances', 'getBalance', { address: options.address, denom: options.denom }, true),
  //   ],
  //   transforms: [],
  // })
  // eventedTasks.push({
  //   ...baseTask,
  //   interval: intervals[1], // immediate
  //   // interval: intervals[4], // 5 blocks
  //   boundary: boundaries(options.currentHeight)[2], // null
  //   actions: [actions(options)[0]], // bank send
  //   queries: [
  //     getQueryMsgByTypes(options.modBalancesAddr, 'balances', 'getBalance', { address: options.address, denom: options.denom }, true),
  //   ],
  //   transforms: [],
  // })
  // eventedTasks.push({
  //   ...baseTask,
  //   interval: intervals[1], // immediate
  //   // interval: intervals[4], // 5 blocks
  //   boundary: boundaries(options.currentHeight)[3], // null
  //   actions: [actions(options)[0]], // bank send
  //   queries: [
  //     getQueryMsgByTypes(options.modBalancesAddr, 'balances', 'getBalance', { address: options.address, denom: options.denom }, true),
  //   ],
  //   transforms: [],
  // })

  // Just QUERY samples
  eventedTasks.push({
    ...baseTask,
    // interval: intervals[1], // immediate
    interval: intervals[4], // 5 blocks
    boundary: boundaries(options.currentHeight)[0], // null
    actions: [actions(options)[0]], // bank send
    queries: [
      getQueryMsgByTypes(options.modBalancesAddr, 'balances', 'getBalance', { address: options.address, denom: options.denom }, true),
    ],
    transforms: [],
  })
  eventedTasks.push({
    ...baseTask,
    interval: intervals[1], // immediate
    boundary: boundaries(options.currentHeight)[3], // 100 block range
    actions: [actions(options)[0]], // bank send
    queries: [
      getQueryMsgByTypes(options.modBalancesAddr, 'balances', 'getBalance', { address: options.address, denom: options.denom }, true),
    ],
    transforms: [],
  })
  // TODO: Need to deploy/initialize cw20 contract for testing
  // eventedTasks.push({
  //   ...baseTask,
  //   interval: intervals[1], // immediate
  //   boundary: boundaries(options.currentHeight)[3], // 100 block range
  //   actions: [actions(options)[1]], // wasm exec tick
  //   queries: [
  //     getQueryMsgByTypes(options.modBalancesAddr, 'balances', 'getCw20Balance', { cw20_contract: options.cw20_contract, denom: options.denom }, true),
  //   ],
  //   transforms: [],
  // })
  // eventedTasks.push({
  //   ...baseTask,
  //   interval: intervals[1], // immediate
  //   boundary: boundaries(options.currentHeight)[3], // 100 block range
  //   actions: [actions(options)[0]], // bank send
  //   queries: [
  //     getQueryMsgByTypes(
  //       options.modBalancesAddr,
  //       'balances',
  //       'getBalanceComparator',
  //       {
  //         address: options.address,
  //         comparator: options.comparator,
  //         required_balance: {
  //           native: [
  //             {
  //               amount: options.amount,
  //               denom: options.denom
  //             }
  //           ]
  //         }
  //       },
  //       true
  //     ),
  //   ],
  //   transforms: [],
  // })

  // QUERY+TRANSFORM samples
  eventedTasks.push({
    ...baseTask,
    interval: intervals[1], // immediate
    boundary: boundaries(options.currentHeight)[3], // 100 block range
    actions: [actions(options)[0]], // bank send
    queries: [
      getQueryMsgByTypes(options.modBalancesAddr, 'balances', 'getBalance', { address: options.address, denom: options.denom }, true),
    ],
    transforms: [
      getTransform(0, 0, 'amount', 'msg.bank.send.amount.amount'),
      getTransform(0, 0, 'denom', 'msg.bank.send.amount.denom'), // not needed, but testing
    ],
  })
  eventedTasks.push({
    ...baseTask,
    interval: intervals[1], // immediate
    boundary: boundaries(options.currentHeight)[3], // 100 block range
    actions: [actions(options)[0]], // bank send
    queries: [
      to_binary(getQueryMsgByTypes(options.modBalancesAddr, 'balances', 'getBalanceComparator', { address: options.address, comparator: options.comparator, required_balance: { amount: options.amount, denom: options.denom } }, true)),
    ],
    transforms: [
      getTransform(0, 0, 'amount', 'msg.bank.send.amount.amount'),
      getTransform(0, 0, 'denom', 'msg.bank.send.amount.denom'), // not needed, but testing
    ],
  })

  return eventedTasks
}

// options = { currentHeight, address, amount, denom }
export const getIntervalTasks = (options: any) => {
  const intervalTasks = []
  const action = actions(options)[0]
  intervals.forEach((int: any) => {
    boundaries(options.currentHeight).forEach((bnd: any) => {
      const task = {
        ...baseTask,
        interval: int,
        boundary: bnd,
        actions: [action],
      }
      if (int.block && bnd && !bnd.time) {
        intervalTasks.push(task)
      }
      if (int.cron && bnd && !bnd.height) {
        intervalTasks.push(task)
      }
    })
  })
  return intervalTasks
}


export const tasks: any = (options: any) => ({
  intervalTasks: getIntervalTasks(options),
  eventedTasks: getEventedTasks(options),
})

export const allTasks = (options: any) => {
  return [].concat(
    getIntervalTasks(options),
    getEventedTasks(options),
  )
}