/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.19.0.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

import { CosmWasmClient, SigningCosmWasmClient, ExecuteResult } from "@cosmjs/cosmwasm-stargate";
import { StdFee } from "@cosmjs/amino";
import { InstantiateMsg, ExecuteMsg, VersionKind, Binary, WasmMsg, Uint128, ModuleInstantiateInfo, Coin, QueryMsg, Addr, ArrayOfEntryResponse, EntryResponse, ContractMetadataInfo, Config, ArrayOfString, ContractMetadataResponse, ArrayOfContractMetadataInfo } from "./CroncatFactory.types";
export interface CroncatFactoryReadOnlyInterface {
  contractAddress: string;
  config: () => Promise<Config>;
  latestContracts: () => Promise<ArrayOfEntryResponse>;
  latestContract: ({
    contractName
  }: {
    contractName: string;
  }) => Promise<ContractMetadataResponse>;
  versionsByContractName: ({
    contractName,
    fromIndex,
    limit
  }: {
    contractName: string;
    fromIndex?: number;
    limit?: number;
  }) => Promise<ArrayOfContractMetadataInfo>;
  contractNames: ({
    fromIndex,
    limit
  }: {
    fromIndex?: number;
    limit?: number;
  }) => Promise<ArrayOfString>;
  allEntries: ({
    fromIndex,
    limit
  }: {
    fromIndex?: number;
    limit?: number;
  }) => Promise<ArrayOfEntryResponse>;
}
export class CroncatFactoryQueryClient implements CroncatFactoryReadOnlyInterface {
  client: CosmWasmClient;
  contractAddress: string;

  constructor(client: CosmWasmClient, contractAddress: string) {
    this.client = client;
    this.contractAddress = contractAddress;
    this.config = this.config.bind(this);
    this.latestContracts = this.latestContracts.bind(this);
    this.latestContract = this.latestContract.bind(this);
    this.versionsByContractName = this.versionsByContractName.bind(this);
    this.contractNames = this.contractNames.bind(this);
    this.allEntries = this.allEntries.bind(this);
  }

  config = async (): Promise<Config> => {
    return this.client.queryContractSmart(this.contractAddress, {
      config: {}
    });
  };
  latestContracts = async (): Promise<ArrayOfEntryResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      latest_contracts: {}
    });
  };
  latestContract = async ({
    contractName
  }: {
    contractName: string;
  }): Promise<ContractMetadataResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      latest_contract: {
        contract_name: contractName
      }
    });
  };
  versionsByContractName = async ({
    contractName,
    fromIndex,
    limit
  }: {
    contractName: string;
    fromIndex?: number;
    limit?: number;
  }): Promise<ArrayOfContractMetadataInfo> => {
    return this.client.queryContractSmart(this.contractAddress, {
      versions_by_contract_name: {
        contract_name: contractName,
        from_index: fromIndex,
        limit
      }
    });
  };
  contractNames = async ({
    fromIndex,
    limit
  }: {
    fromIndex?: number;
    limit?: number;
  }): Promise<ArrayOfString> => {
    return this.client.queryContractSmart(this.contractAddress, {
      contract_names: {
        from_index: fromIndex,
        limit
      }
    });
  };
  allEntries = async ({
    fromIndex,
    limit
  }: {
    fromIndex?: number;
    limit?: number;
  }): Promise<ArrayOfEntryResponse> => {
    return this.client.queryContractSmart(this.contractAddress, {
      all_entries: {
        from_index: fromIndex,
        limit
      }
    });
  };
}
export interface CroncatFactoryInterface extends CroncatFactoryReadOnlyInterface {
  contractAddress: string;
  sender: string;
  deploy: ({
    kind,
    moduleInstantiateInfo
  }: {
    kind: VersionKind;
    moduleInstantiateInfo: ModuleInstantiateInfo;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  remove: ({
    contractName,
    version
  }: {
    contractName: string;
    version: number[];
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  updateMetadata: ({
    changelogUrl,
    contractName,
    schema,
    version
  }: {
    changelogUrl?: string;
    contractName: string;
    schema?: string;
    version: number[];
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  proxy: ({
    msg
  }: {
    msg: WasmMsg;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  nominateOwner: ({
    nominatedOwnerAddr
  }: {
    nominatedOwnerAddr: string;
  }, fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  acceptNominateOwner: (fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
  removeNominateOwner: (fee?: number | StdFee | "auto", memo?: string, funds?: Coin[]) => Promise<ExecuteResult>;
}
export class CroncatFactoryClient extends CroncatFactoryQueryClient implements CroncatFactoryInterface {
  client: SigningCosmWasmClient;
  sender: string;
  contractAddress: string;

  constructor(client: SigningCosmWasmClient, sender: string, contractAddress: string) {
    super(client, contractAddress);
    this.client = client;
    this.sender = sender;
    this.contractAddress = contractAddress;
    this.deploy = this.deploy.bind(this);
    this.remove = this.remove.bind(this);
    this.updateMetadata = this.updateMetadata.bind(this);
    this.proxy = this.proxy.bind(this);
    this.nominateOwner = this.nominateOwner.bind(this);
    this.acceptNominateOwner = this.acceptNominateOwner.bind(this);
    this.removeNominateOwner = this.removeNominateOwner.bind(this);
  }

  deploy = async ({
    kind,
    moduleInstantiateInfo
  }: {
    kind: VersionKind;
    moduleInstantiateInfo: ModuleInstantiateInfo;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      deploy: {
        kind,
        module_instantiate_info: moduleInstantiateInfo
      }
    }, fee, memo, funds);
  };
  remove = async ({
    contractName,
    version
  }: {
    contractName: string;
    version: number[];
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      remove: {
        contract_name: contractName,
        version
      }
    }, fee, memo, funds);
  };
  updateMetadata = async ({
    changelogUrl,
    contractName,
    schema,
    version
  }: {
    changelogUrl?: string;
    contractName: string;
    schema?: string;
    version: number[];
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      update_metadata: {
        changelog_url: changelogUrl,
        contract_name: contractName,
        schema,
        version
      }
    }, fee, memo, funds);
  };
  proxy = async ({
    msg
  }: {
    msg: WasmMsg;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      proxy: {
        msg
      }
    }, fee, memo, funds);
  };
  nominateOwner = async ({
    nominatedOwnerAddr
  }: {
    nominatedOwnerAddr: string;
  }, fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      nominate_owner: {
        nominated_owner_addr: nominatedOwnerAddr
      }
    }, fee, memo, funds);
  };
  acceptNominateOwner = async (fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      accept_nominate_owner: {}
    }, fee, memo, funds);
  };
  removeNominateOwner = async (fee: number | StdFee | "auto" = "auto", memo?: string, funds?: Coin[]): Promise<ExecuteResult> => {
    return await this.client.execute(this.sender, this.contractAddress, {
      remove_nominate_owner: {}
    }, fee, memo, funds);
  };
}