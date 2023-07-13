import { SafeHtml } from '@angular/platform-browser';
import { Feature, Server } from '../servers/types';
import { ActionDefinition } from '../plugins/types';
import { User } from '../users/types';

export class RowData {
  constructor(
    public server: Server,
    public ipaddress: string,
    public name: string,
    public dnsname: string = '',
    public version: number,
    public show_details: boolean = false,
  ) {}
}

export class GUIAction {
  constructor(
    public feature: Feature,
    public action: ActionDefinition,
    public icon: SafeHtml | undefined,
    public needs_confirmation: boolean
  ) {}
}

export class DNSServer {
  constructor(public ipaddress: string, public port: number) {}
}

export class SystemInformation {
  constructor(
    public memory_stats: SystemInformationEntry[],
    public memory_usage: SystemInformationEntry[],
    public load_average: SystemInformationEntry[]
  ) {}
}

export class SystemInformationEntry {
  constructor(public name: string, public value: number) {}
}

export class Configuration {
  constructor(
    public disabled_plugins: string[],
    public users: User[],
    public servers: Server[],
    public dns_servers: DNSServer[]
  ) {}
}
