/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.19.0.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

import { CosmWasmClient, SigningCosmWasmClient, ExecuteResult } from "@cosmjs/cosmwasm-stargate";
import { Coin, StdFee } from "@cosmjs/amino";
import { Addr, InstantiateMsg, ExecuteMsg, AgentOnTaskCreated, AgentOnTaskCompleted, UpdateConfig, QueryMsg, Config, Uint128, Uint64, Timestamp, AgentStatus, AgentResponse, AgentInfo, GetAgentIdsResponse, AgentTaskResponse, TaskStats, ApprovedAgentAddresses, Boolean } from "./CroncatAgents.types";
export interface CroncatAgentsReadOnlyInterface {
  contractAddress: string;
  getAgent: ({
    accountId
  }: {
    accountId: string;
  }) => Promise<AgentResponse>;
  getAgentIds: ({
    fromIndex,
    limit
  }: {
    fromIndex?: number;
    limit?: number;
  }) => Promise<GetAgentIdsResponse>;
  getApprovedAgentAddresses: ({
    fromIndex,
    limit
  }: {
    fromIndex?: number;
    limit?: number;
  }) => Promise<ApprovedAgentAddresses>;
  getAgentTasks: ({
    accountId
  }: {
    accountId: string;
  }) => Promise<AgentTaskResponse>;
  config: () => Promise<Config>;
  paused: () => Promise<Boolean>;
}
export class CroncatAgentsQueryClient implements CroncatAgentsReadOnlyInterface {
  client: CosmWasmClient;
  contractAddress: string;

  constructor(client: CosmWasmClient, contractAddress: string) {
    this.client = client;
    this.contractAddress = contractAddress;
    this.getAgent = this.getAgent.bind(this);
    this.getAgentIds = this.getAgentIds.bind(this);
    this.getApprovedAgentAddresses = this.getApprovedAgentAddresses.bind(this);
    this.getAgentTasks = this.getAgentTasks.bind(this);
    this.config = this.config.bind(this);
    this.paused = this.paused.bind(this);
  }

  getAgent = async ({
    accountId
  }: {
    accountId: string;
  }): Promise<AgentResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      get_agent: {
        account_id: accountId
      }
    });
  };
  getAgentIds = async ({
    fromIndex,
    limit
  }: {
    fromIndex?: number;
    limit?: number;
  }): Promise<GetAgentIdsResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      get_agent_ids: {
        from_index: fromIndex,
        limit
      }
    });
  };
  getApprovedAgentAddresses = async ({
    fromIndex,
    limit
  }: {
    fromIndex?: number;
    limit?: number;
  }): Promise<ApprovedAgentAddresses> => {
    return this.client.queryContractSmart(this.contractAddress, {
      get_approved_agent_addresses: {
        from_index: fromIndex,
        limit
      }
    });
  };
  getAgentTasks = async ({
    accountId
  }: {
    accountId: string;
  }): Promise<AgentTaskResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      get_agent_tasks: {
        account_id: accountId
      }
    });
  };
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
}
export interface CroncatAgentsInterface extends CroncatAgentsReadOnlyInterface {
  contractAddress: string;
  sender: string;
  addAgentToWhitelist: ({
    agentAddress
  }: {
    agentAddress: string;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  removeAgentFromWhitelist: ({
    agentAddress
  }: {
    agentAddress: string;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  registerAgent: ({
    payableAccountId
  }: {
    payableAccountId?: string;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  updateAgent: ({
    payableAccountId
  }: {
    payableAccountId: string;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  checkInAgent: (fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  unregisterAgent: ({
    fromBehind
  }: {
    fromBehind?: boolean;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  onTaskCreated: (fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  onTaskCompleted: ({
    agentId,
    isBlockSlotTask
  }: {
    agentId: Addr;
    isBlockSlotTask: boolean;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  updateConfig: ({
    config
  }: {
    config: UpdateConfig;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  tick: (fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  pauseContract: (fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  unpauseContract: (fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
}
export class CroncatAgentsClient extends CroncatAgentsQueryClient implements CroncatAgentsInterface {
  client: SigningCosmWasmClient;
  sender: string;
  contractAddress: string;

  constructor(client: SigningCosmWasmClient, sender: string, contractAddress: string) {
    super(client, contractAddress);
    this.client = client;
    this.sender = sender;
    this.contractAddress = contractAddress;
    this.addAgentToWhitelist = this.addAgentToWhitelist.bind(this);
    this.removeAgentFromWhitelist = this.removeAgentFromWhitelist.bind(this);
    this.registerAgent = this.registerAgent.bind(this);
    this.updateAgent = this.updateAgent.bind(this);
    this.checkInAgent = this.checkInAgent.bind(this);
    this.unregisterAgent = this.unregisterAgent.bind(this);
    this.onTaskCreated = this.onTaskCreated.bind(this);
    this.onTaskCompleted = this.onTaskCompleted.bind(this);
    this.updateConfig = this.updateConfig.bind(this);
    this.tick = this.tick.bind(this);
    this.pauseContract = this.pauseContract.bind(this);
    this.unpauseContract = this.unpauseContract.bind(this);
  }

  addAgentToWhitelist = async ({
    agentAddress
  }: {
    agentAddress: string;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      add_agent_to_whitelist: {
        agent_address: agentAddress
      }
    }, fee, memo, funds);
  };
  removeAgentFromWhitelist = async ({
    agentAddress
  }: {
    agentAddress: string;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      remove_agent_from_whitelist: {
        agent_address: agentAddress
      }
    }, fee, memo, funds);
  };
  registerAgent = async ({
    payableAccountId
  }: {
    payableAccountId?: string;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      register_agent: {
        payable_account_id: payableAccountId
      }
    }, fee, memo, funds);
  };
  updateAgent = async ({
    payableAccountId
  }: {
    payableAccountId: string;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      update_agent: {
        payable_account_id: payableAccountId
      }
    }, fee, memo, funds);
  };
  checkInAgent = async (fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      check_in_agent: {}
    }, fee, memo, funds);
  };
  unregisterAgent = async ({
    fromBehind
  }: {
    fromBehind?: boolean;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      unregister_agent: {
        from_behind: fromBehind
      }
    }, fee, memo, funds);
  };
  onTaskCreated = async (fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      on_task_created: {}
    }, fee, memo, funds);
  };
  onTaskCompleted = async ({
    agentId,
    isBlockSlotTask
  }: {
    agentId: Addr;
    isBlockSlotTask: boolean;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      on_task_completed: {
        agent_id: agentId,
        is_block_slot_task: isBlockSlotTask
      }
    }, fee, memo, funds);
  };
  updateConfig = async ({
    config
  }: {
    config: UpdateConfig;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      update_config: {
        config
      }
    }, fee, memo, funds);
  };
  tick = async (fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      tick: {}
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