import { SafeHtml } from '@angular/platform-browser';
import { Feature, Server } from '../servers/types';
import { Action } from '../plugins/types';

export class RowData {
  constructor(
    public server: Server,
    public ipaddress: string,
    public name: string,
    public dnsname: string = '',
    public show_details: boolean = false
  ) {}
}

export class GUIAction {
  constructor(
    public feature: Feature,
    public action: Action,
    public icon: SafeHtml | undefined,
    public needs_confirmation: boolean
  ) {}
}

export class ConfigAction {
  constructor(
    public action_type: 'SaveDNSServers',
    dnsServers: DNSServer[] = []
  ) {}
}

export class DNSServer {
  constructor(public ipaddress: string, public port: number) {}
}

export class SystemInformation {
  constructor( public mem_stats: SystemInformationEntry[],  public memory_usage: SystemInformationEntry[],  public load_average: SystemInformationEntry[]) {};
}

export class SystemInformationEntry {
    constructor(public name: string, public value: number) {}
}
