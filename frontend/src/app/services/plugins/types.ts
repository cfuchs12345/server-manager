export class Param {
  constructor(public name: string, public value: string) {}
}

export class PluginsAction {
  constructor(public action_type: 'Disable', public params: Param[]) {}
}

// Plugin types

export class Script {
  constructor(public script_type: string, public script: string) {}
}

export class CredentialDefinition {
  constructor(
    public name: string,
    public credential_type: string,
    public encrypt: boolean,
    public default_value: string,
    public mandatory: boolean = false
  ) {}
}

export class ParamDefinition {
  constructor(
    public name: string,
    public param_type: string,
    public default_value: string,
    public mandatory: boolean = false
  ) {}
}

export class DependsDef {
  constructor(public data_id: string) {}
}

export class ActionDefinition {
  constructor(
    public id: string,
    public name: string,
    public show_on_main: boolean = true,
    public description: string,
    public icon: string,
    public needs_confirmation: boolean = true,
    public available_for_state: 'Any' | 'Inactive' | 'Active' = 'Any',
    public depends: DependsDef[]
  ) {}
}

export class DetectionDefinition {
  constructor(
    public ports: number[],
    public script: Script[],
    public detection_possible: boolean
  ) {}
}

export class NotificationDefinition {
  constructor(

  ) {
  }
}

export class Plugin {
  constructor(
    public id: string,
    public name: string,
    public description: string,
    public server_icon: string,
    public detection: DetectionDefinition,
    public credentials: CredentialDefinition[],
    public params: ParamDefinition[],
    public notification: NotificationDefinition[],
    public actions: ActionDefinition[]
  ) {}
}
