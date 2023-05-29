import { Component, OnDestroy, OnInit } from '@angular/core';
import { FormControl, Validators } from '@angular/forms';
import { ServerService } from '../../../../services/servers/server.service';
import { HostInformation, Server } from '../../../../services/servers/types';
import { MatDialogRef } from '@angular/material/dialog';
import { AutoDiscoveryDialog } from '../dialog-autodiscover';
import { Subscription } from 'rxjs';
import { ServerDiscoveryService } from 'src/app/services/servers/server-discovery.service';
import { GeneralService } from 'src/app/services/general/general.service';
import { DNSServer } from 'src/app/services/general/types';
import { ErrorService } from 'src/app/services/errors/error.service';

@Component({
  selector: 'app-autodiscover-server-modal',
  templateUrl: './autodiscover-server-modal.component.html',
  styleUrls: ['./autodiscover-server-modal.component.scss'],
})
export class AutodiscoverServerModalComponent implements OnInit, OnDestroy {
  buttonTextStart: string = 'Start';
  buttonTextWorking: string = 'Working...';
  buttonTextSaveServer: string = 'Save Servers';
  inputHintNetworkmask: string = 'Enter the network using CIDR notatation';
  inputExampleNetworkmask: string = 'Example: 192.168.178.0/24';
  inputPlaceholderNetworkmask: string = 'xxx.xxx.xxx.xxx/xx';
  inputPatternNetworkmask: string = '^([0-9]{1,3}.){3}[0-9]{1,3}/([0-9]|[1-2][0-9]|3[0-2])$';
  formControlNetworkmask = new FormControl('', [
    Validators.required,
    Validators.pattern(this.inputPatternNetworkmask),
  ]);
  loading: boolean = false;

  displayedColumns = ['selected', 'ipaddress', 'dnsname', 'running'];

  existing_servers: Server[] = [];
  servers: HostInformation[] = [];
  dnsservers: DNSServer[] = [];

  subscriptionServers: Subscription | undefined = undefined;
  subscriptionDNSServers: Subscription | undefined = undefined;
  subscriptionExistingServers: Subscription | undefined = undefined;

  constructor(
    private serverService: ServerService,
    private discoverService: ServerDiscoveryService,
    private generalService: GeneralService,
    private errorService: ErrorService,
    private ref: MatDialogRef<AutoDiscoveryDialog>
  ) {
  }

  ngOnInit(): void {
    this.subscriptionDNSServers = this.generalService.dnsServers.subscribe((dnsservers) => {
      this.dnsservers = dnsservers;
    });

    this.subscriptionServers = this.discoverService.discoveredServers.subscribe((servers) => {
      if (servers) {
        let retained_list = servers.filter(  s => !this.existing_servers.find( e => e.ipaddress === s.ipaddress)  );

        for (let i = 0; i < retained_list.length; i++) {
          retained_list[i].selected = true;
        }
        this.servers = retained_list;
      } else {
        // clear messages when empty message received
        this.servers = [];
      }
      this.loading = false;
    });
    this.subscriptionExistingServers = this.serverService.servers.subscribe((servers) =>{
      this.existing_servers = servers;
    });
    this.generalService.listDNSServers();
    this.serverService.listServers();
  }


  // doesn't seem to work when written as arrow function!?
  ngOnDestroy(): void {
    if( this.subscriptionServers) {
      this.subscriptionServers.unsubscribe();
    }
    if( this.subscriptionDNSServers ) {
      this.subscriptionDNSServers.unsubscribe();
    }
    if( this.subscriptionExistingServers ) {
      this.subscriptionExistingServers.unsubscribe();
    }
    this.discoverService.resetDiscoveredServers();
  }



  serversFound = (): boolean => {
    return this.servers.length > 0;
  }

  getErrorMessageNetworkMask = () => {
    if (this.formControlNetworkmask.hasError('required')) {
      return 'You must enter a value';
    }
    return this.formControlNetworkmask.hasError('pattern')
      ? 'The network mask format is not correct'
      : 'Unknown error';
  };

  onClickAutoDiscover = () => {
    if( !this.dnsservers || this.dnsservers.length == 0 ) {
      this.errorService.newError("Auto Discovery", undefined, "Cannot run autodovery. No DNS Server configured.");
      return;
    }
    const value = this.formControlNetworkmask.getRawValue();

    if (value != null) {
      this.loading = true;
      this.discoverService.autoDiscoverServers(value, true);
    }
  };

  onClickSaveServers = () => {
    var serversToSave: Server[] = [];
    for (let i = 0; i < this.servers.length; i++) {
      var server = this.servers[i];

      if (server.selected) {
        serversToSave.push(
          new Server(server.ipaddress, '', server.dnsname, [])
        );
      }
    }
    this.serverService.saveServers(serversToSave);
    setTimeout(this.serverService.listServers, 20);
    this.ref.close();
  };

  onClickDeselectServer = (index: number) => {
    this.servers[index].selected = !this.servers[index].selected;
  };
}
