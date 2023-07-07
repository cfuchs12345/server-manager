export class Param {
  constructor(public name: string, public value: string) {}
}

export class Credential {
  constructor(
    public name: string,
    public value: string,
    public encrypted: boolean
  ) {}
}

// types for server discovery
export class NetworksAction {
  constructor(
    public action_type: 'AutoDiscover',
    public params: Param[] = []
  ) {}
}

export class HostInformation {
  constructor(
    public is_running: boolean,
    public ipaddress: string,
    public dnsname: string,
    public selected: boolean = true
  ) {}
}

// Status Query

export class ServersAction {
  constructor(
    public action_type: 'Status' | 'FeatureScan' | 'ActionConditionCheck',
    public params: Param[] = []
  ) {}
}

export class Status {
  constructor(public ipaddress: string, public is_running: boolean) {}
}

export class ServerFeature {
  constructor(
    public ipaddress: string,
    public features: Feature[] = [],
    public selected: boolean = true
  ) {}
}

// server types
export class Feature {
  constructor(
    public id: string,
    public name: string,
    public params: Param[],
    public credentials: Credential[]
  ) {}
}

export class Server {
  constructor(
    public ipaddress: string,
    public name: string,
    public dnsname: string = '',
    public features: Feature[] = [],
    public isPreliminary: boolean = false,
    public change_flag: string = ""
  ) {}
}

export class ServerAction {
  constructor(
    public action_type:
      | 'FeatureScan'
      | 'ExecuteFeatureAction'
      | 'Status'
      | 'QueryData'
      | 'ActionConditionCheck'
      | 'SubActionConditionCheck',
    public params: Param[] = [],
    public condition_checks: SubActionConditionCheck[] = []
  ) {}
}

export class DataResult {
  constructor(
    public ipaddress: string,
    public data_id: string,
    public result: string,
    public check_results: ConditionCheckResult[]
  ) {}
}

export class SubActionConditionCheck {
  constructor(
    public feature_id: string,
    public action_id: string,
    public data_id: string,
    public action_params: string | undefined = undefined
  ) {}
}

export class ConditionCheckResult {
  constructor(
    public ipaddress: string,
    public data_id: string = "",
    public subresults: ConditionCheckSubResult[]
  ) {}
}

export class ConditionCheckSubResult {
  constructor(
    public action_id: string,
    public action_params: string,
    public feature_id: string,
    public result: boolean
  ) {}
}

// util methods for those types

export const getIpAddressesFromServers = (servers: Server[]) => {
  const ipaddresses: string[] = [];
  servers.forEach((server) => {
    ipaddresses.push(server.ipaddress);
  });
  return ipaddresses;
};
