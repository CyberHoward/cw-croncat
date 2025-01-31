/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.19.0.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

import { CosmWasmClient, SigningCosmWasmClient, ExecuteResult } from "@cosmjs/cosmwasm-stargate";
import { StdFee } from "@cosmjs/amino";
import { Addr, InstantiateMsg, ExecuteMsg, CosmosMsgForEmpty, BankMsg, Uint128, StakingMsg, DistributionMsg, Binary, IbcMsg, Timestamp, Uint64, WasmMsg, GovMsg, VoteOption, Boundary, Interval, CosmosQueryForWasmQuery, WasmQuery, ValueIndex, PathToValue, UpdateConfigMsg, TaskRequest, ActionForEmpty, Coin, Empty, IbcTimeout, IbcTimeoutBlock, BoundaryHeight, BoundaryTime, Cw20Coin, CroncatQuery, Transform, TasksRemoveTaskByManager, TasksRescheduleTask, QueryMsg, Task, AmountForOneTask, Cw20CoinVerified, GasPrice, Config, TaskResponse, TaskInfo, CurrentTaskInfoResponse, ArrayOfString, ArrayOfUint64, ArrayOfTaskInfo, Boolean, SlotHashesResponse, SlotIdsResponse, SlotTasksTotalResponse, String } from "./CroncatTasks.types";
export interface CroncatTasksReadOnlyInterface {
  contractAddress: string;
  config: () => Promise<Config>;
  paused: () => Promise<Boolean>;
  tasksTotal: () => Promise<Uint64>;
  currentTaskInfo: () => Promise<CurrentTaskInfoResponse>;
  currentTask: () => Promise<TaskResponse>;
  task: ({
    taskHash
  }: {
    taskHash: string;
  }) => Promise<TaskResponse>;
  tasks: ({
    fromIndex,
    limit
  }: {
    fromIndex?: number;
    limit?: number;
  }) => Promise<ArrayOfTaskInfo>;
  eventedIds: ({
    fromIndex,
    limit
  }: {
    fromIndex?: number;
    limit?: number;
  }) => Promise<ArrayOfUint64>;
  eventedHashes: ({
    fromIndex,
    id,
    limit
  }: {
    fromIndex?: number;
    id?: number;
    limit?: number;
  }) => Promise<ArrayOfString>;
  eventedTasks: ({
    fromIndex,
    limit,
    start
  }: {
    fromIndex?: number;
    limit?: number;
    start?: number;
  }) => Promise<ArrayOfTaskInfo>;
  tasksByOwner: ({
    fromIndex,
    limit,
    ownerAddr
  }: {
    fromIndex?: number;
    limit?: number;
    ownerAddr: string;
  }) => Promise<ArrayOfTaskInfo>;
  taskHash: ({
    task
  }: {
    task: Task;
  }) => Promise<String>;
  slotHashes: ({
    slot
  }: {
    slot?: number;
  }) => Promise<SlotHashesResponse>;
  slotIds: ({
    fromIndex,
    limit
  }: {
    fromIndex?: number;
    limit?: number;
  }) => Promise<SlotIdsResponse>;
  slotTasksTotal: ({
    offset
  }: {
    offset?: number;
  }) => Promise<SlotTasksTotalResponse>;
}
export class CroncatTasksQueryClient implements CroncatTasksReadOnlyInterface {
  client: CosmWasmClient;
  contractAddress: string;

  constructor(client: CosmWasmClient, contractAddress: string) {
    this.client = client;
    this.contractAddress = contractAddress;
    this.config = this.config.bind(this);
    this.paused = this.paused.bind(this);
    this.tasksTotal = this.tasksTotal.bind(this);
    this.currentTaskInfo = this.currentTaskInfo.bind(this);
    this.currentTask = this.currentTask.bind(this);
    this.task = this.task.bind(this);
    this.tasks = this.tasks.bind(this);
    this.eventedIds = this.eventedIds.bind(this);
    this.eventedHashes = this.eventedHashes.bind(this);
    this.eventedTasks = this.eventedTasks.bind(this);
    this.tasksByOwner = this.tasksByOwner.bind(this);
    this.taskHash = this.taskHash.bind(this);
    this.slotHashes = this.slotHashes.bind(this);
    this.slotIds = this.slotIds.bind(this);
    this.slotTasksTotal = this.slotTasksTotal.bind(this);
  }

  config = async (): Promise<Config> => {
    return this.client.queryContractSmart(this.contractAddress, {
      config: {}
    });
  };
  paused = async (): Promise<Boolean> => {
    return this.client.queryContractSmart(this.contractAddress, {
      paused: {}
    });
  };
  tasksTotal = async (): Promise<Uint64> => {
    return this.client.queryContractSmart(this.contractAddress, {
      tasks_total: {}
    });
  };
  currentTaskInfo = async (): Promise<CurrentTaskInfoResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      current_task_info: {}
    });
  };
  currentTask = async (): Promise<TaskResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      current_task: {}
    });
  };
  task = async ({
    taskHash
  }: {
    taskHash: string;
  }): Promise<TaskResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      task: {
        task_hash: taskHash
      }
    });
  };
  tasks = async ({
    fromIndex,
    limit
  }: {
    fromIndex?: number;
    limit?: number;
  }): Promise<ArrayOfTaskInfo> => {
    return this.client.queryContractSmart(this.contractAddress, {
      tasks: {
        from_index: fromIndex,
        limit
      }
    });
  };
  eventedIds = async ({
    fromIndex,
    limit
  }: {
    fromIndex?: number;
    limit?: number;
  }): Promise<ArrayOfUint64> => {
    return this.client.queryContractSmart(this.contractAddress, {
      evented_ids: {
        from_index: fromIndex,
        limit
      }
    });
  };
  eventedHashes = async ({
    fromIndex,
    id,
    limit
  }: {
    fromIndex?: number;
    id?: number;
    limit?: number;
  }): Promise<ArrayOfString> => {
    return this.client.queryContractSmart(this.contractAddress, {
      evented_hashes: {
        from_index: fromIndex,
        id,
        limit
      }
    });
  };
  eventedTasks = async ({
    fromIndex,
    limit,
    start
  }: {
    fromIndex?: number;
    limit?: number;
    start?: number;
  }): Promise<ArrayOfTaskInfo> => {
    return this.client.queryContractSmart(this.contractAddress, {
      evented_tasks: {
        from_index: fromIndex,
        limit,
        start
      }
    });
  };
  tasksByOwner = async ({
    fromIndex,
    limit,
    ownerAddr
  }: {
    fromIndex?: number;
    limit?: number;
    ownerAddr: string;
  }): Promise<ArrayOfTaskInfo> => {
    return this.client.queryContractSmart(this.contractAddress, {
      tasks_by_owner: {
        from_index: fromIndex,
        limit,
        owner_addr: ownerAddr
      }
    });
  };
  taskHash = async ({
    task
  }: {
    task: Task;
  }): Promise<String> => {
    return this.client.queryContractSmart(this.contractAddress, {
      task_hash: {
        task
      }
    });
  };
  slotHashes = async ({
    slot
  }: {
    slot?: number;
  }): Promise<SlotHashesResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      slot_hashes: {
        slot
      }
    });
  };
  slotIds = async ({
    fromIndex,
    limit
  }: {
    fromIndex?: number;
    limit?: number;
  }): Promise<SlotIdsResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      slot_ids: {
        from_index: fromIndex,
        limit
      }
    });
  };
  slotTasksTotal = async ({
    offset
  }: {
    offset?: number;
  }): Promise<SlotTasksTotalResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      slot_tasks_total: {
        offset
      }
    });
  };
}
export interface CroncatTasksInterface extends CroncatTasksReadOnlyInterface {
  contractAddress: string;
  sender: string;
  updateConfig: ({
    croncatAgentsKey,
    croncatFactoryAddr,
    croncatManagerKey,
    gasActionFee,
    gasBaseFee,
    gasLimit,
    gasQueryFee,
    slotGranularityTime
  }: {
    croncatAgentsKey?: string[][];
    croncatFactoryAddr?: string;
    croncatManagerKey?: string[][];
    gasActionFee?: number;
    gasBaseFee?: number;
    gasLimit?: number;
    gasQueryFee?: number;
    slotGranularityTime?: number;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  createTask: ({
    task
  }: {
    task: TaskRequest;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  removeTask: ({
    taskHash
  }: {
    taskHash: string;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  removeTaskByManager: ({
    taskHash
  }: {
    taskHash: number[];
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  rescheduleTask: ({
    taskHash
  }: {
    taskHash: number[];
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  pauseContract: (fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  unpauseContract: (fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
}
export class CroncatTasksClient extends CroncatTasksQueryClient implements CroncatTasksInterface {
  client: SigningCosmWasmClient;
  sender: string;
  contractAddress: string;

  constructor(client: SigningCosmWasmClient, sender: string, contractAddress: string) {
    super(client, contractAddress);
    this.client = client;
    this.sender = sender;
    this.contractAddress = contractAddress;
    this.updateConfig = this.updateConfig.bind(this);
    this.createTask = this.createTask.bind(this);
    this.removeTask = this.removeTask.bind(this);
    this.removeTaskByManager = this.removeTaskByManager.bind(this);
    this.rescheduleTask = this.rescheduleTask.bind(this);
    this.pauseContract = this.pauseContract.bind(this);
    this.unpauseContract = this.unpauseContract.bind(this);
  }

  updateConfig = async ({
    croncatAgentsKey,
    croncatFactoryAddr,
    croncatManagerKey,
    gasActionFee,
    gasBaseFee,
    gasLimit,
    gasQueryFee,
    slotGranularityTime
  }: {
    croncatAgentsKey?: string[][];
    croncatFactoryAddr?: string;
    croncatManagerKey?: string[][];
    gasActionFee?: number;
    gasBaseFee?: number;
    gasLimit?: number;
    gasQueryFee?: number;
    slotGranularityTime?: number;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      update_config: {
        croncat_agents_key: croncatAgentsKey,
        croncat_factory_addr: croncatFactoryAddr,
        croncat_manager_key: croncatManagerKey,
        gas_action_fee: gasActionFee,
        gas_base_fee: gasBaseFee,
        gas_limit: gasLimit,
        gas_query_fee: gasQueryFee,
        slot_granularity_time: slotGranularityTime
      }
    }, fee, memo, funds);
  };
  createTask = async ({
    task
  }: {
    task: TaskRequest;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      create_task: {
        task
      }
    }, fee, memo, funds);
  };
  removeTask = async ({
    taskHash
  }: {
    taskHash: string;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      remove_task: {
        task_hash: taskHash
      }
    }, fee, memo, funds);
  };
  removeTaskByManager = async ({
    taskHash
  }: {
    taskHash: number[];
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      remove_task_by_manager: {
        task_hash: taskHash
      }
    }, fee, memo, funds);
  };
  rescheduleTask = async ({
    taskHash
  }: {
    taskHash: number[];
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      reschedule_task: {
        task_hash: taskHash
      }
    }, fee, memo, funds);
  };
  pauseContract = async (fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      pause_contract: {}
    }, fee, memo, funds);
  };
  unpauseContract = async (fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      unpause_contract: {}
    }, fee, memo, funds);
  };
}