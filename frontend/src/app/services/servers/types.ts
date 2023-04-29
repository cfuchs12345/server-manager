


export class Param {
  constructor(public name: string, public value: string) {}
}

export class Credential {
  constructor(public name: string, public value: string, public is_encrypted: boolean) {}
}

// types for server discovery
export class NetworksAction {
  constructor(public action_type: 'AutoDiscover', public params: Param[] = []) {}
}

export class HostInformation {
  constructor(public is_running: boolean, public ipaddress: string, public dnsname: string, public selected: boolean = true) {}
}


// Status Query

export class ServersAction {
  constructor(public action_type: 'Status' | 'FeatureScan', public params: Param[] = []) {}
}

export class Status {
  constructor(public ipaddress: string, public is_running: boolean | undefined = undefined) {}
}


export class ServerFeature {
  constructor(public ipaddress: string, public features: Feature[] = [], public selected: boolean = true) {}
}





// server types
export class Feature {
  constructor(public id: string, public name: string, public params: Param[], public credentials: Credential[]) {}
}

export class Server {
  constructor(public ipaddress: string, public name: string, public dnsname: string = '', public features: Feature[] = [], public selected = false) {}
}




export class ServerAction {
  constructor(public action_type: 'FeatureScan' | 'ExecuteFeatureAction' | 'Status' | 'QueryData' | 'ActionConditionCheck', public params: Param[] = []) {}
}

export class DataResult {
  constructor(public timeStamp: Date, public results: string[]) {};
}

export class ConditionCheck {
  constructor(public ipaddress: string, public feature_id: string, public action_id: string) {}
}
export class ConditionCheckResult {
  constructor(public ipaddress: string, public feature_id: string, public action_id: string, public result: boolean) {}
}

export const newConditionCheckResultFromCheck = (check: ConditionCheck, result: boolean) => {
  return new ConditionCheckResult(check.ipaddress, check.feature_id, check.action_id, result);
}


// util methods for those types

export const getIpAddressesFromServers = (servers: Server[]) => {
  var ipaddresses: string[] = [];
  servers.forEach((server) => {
    ipaddresses.push(server.ipaddress);
  });
  return ipaddresses;
};
