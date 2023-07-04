import { Component, OnDestroy, OnInit } from '@angular/core';
import { FormControl, Validators } from '@angular/forms';
import { ServerService } from '../../../../services/servers/server.service';
import { HostInformation, Server } from '../../../../services/servers/types';
import { MatDialogRef } from '@angular/material/dialog';
import { AutoDiscoveryDialog } from '../dialog-autodiscover';
import { Subscription, filter } from 'rxjs';
import { ServerDiscoveryService } from 'src/app/services/servers/server-discovery.service';
import { GeneralService } from 'src/app/services/general/general.service';
import { DNSServer } from 'src/app/services/general/types';
import { ErrorService, Source } from 'src/app/services/errors/error.service';
import { RxwebValidators, IpVersion } from '@rxweb/reactive-form-validators';
import { Store } from '@ngrx/store';
import { selectAllServers } from 'src/app/state/server/server.selectors';

@Component({
  selector: 'app-autodiscover-server-modal',
  templateUrl: './autodiscover-server-modal.component.html',
  styleUrls: ['./autodiscover-server-modal.component.scss'],
})
export class AutodiscoverServerModalComponent implements OnInit, OnDestroy {
  buttonTextStart = 'Start';
  buttonTextWorking = 'Working...';
  buttonTextSaveServer = 'Save Servers';
  inputHintNetworkmask = 'Enter the network using CIDR notatation';
  inputExampleNetworkmask = 'Example: 192.168.178.0/24';
  inputPlaceholderNetworkmask = 'xxx.xxx.xxx.xxx/xx';

  formControlNetworkmask = new FormControl('', [
    Validators.required,
    RxwebValidators.ip({ version: IpVersion.AnyOne, isCidr: true }),
  ]);
  loading = false;

  displayedColumns = ['selected', 'ipaddress', 'dnsname', 'running'];

  existing_servers: Server[] = [];
  servers: HostInformation[] = [];
  dnsservers: DNSServer[] = [];

  subscriptionServers: Subscription | undefined = undefined;
  subscriptionExistingServers: Subscription | undefined = undefined;

  constructor(
    private store: Store,
    private serverService: ServerService,
    private discoverService: ServerDiscoveryService,
    private generalService: GeneralService,
    private errorService: ErrorService,
    private ref: MatDialogRef<AutoDiscoveryDialog>
  ) {}

  ngOnInit(): void {
    this.loadDNSServers();

    this.subscriptionExistingServers = this.store
      .select(selectAllServers)
      .subscribe((servers) => {
        this.existing_servers = servers;
      });
  }

  private loadDNSServers() {
    const subscriptionDNSServers = this.generalService
      .listDNSServers()
      .subscribe((dnsservers) => {
        this.dnsservers = dnsservers;
        subscriptionDNSServers.unsubscribe();
      });
  }

  // doesn't seem to work when written as arrow function!?
  ngOnDestroy(): void {
    if (this.subscriptionServers) {
      this.subscriptionServers.unsubscribe();
    }
    if (this.subscriptionExistingServers) {
      this.subscriptionExistingServers.unsubscribe();
    }
  }

  serversFound = (): boolean => {
    return this.servers.length > 0;
  };

  getErrorMessageNetworkMask = () => {
    if (this.formControlNetworkmask.hasError('required')) {
      return 'You must enter a value';
    }
    return this.formControlNetworkmask.hasError('ip')
      ? 'The network mask format is not correct'
      : 'Unknown error';
  };

  onClickAutoDiscover = () => {
    if (!this.dnsservers || this.dnsservers.length == 0) {
      this.errorService.newError(
        Source.AutodiscoverServerModalComponent,
        undefined,
        'Cannot run autodovery. No DNS Server configured.'
      );
      return;
    }
    const value = this.formControlNetworkmask.getRawValue();

    if (value != null) {
      this.loading = true;
      this.discoverService
        .autoDiscoverServers(value, true)
        .subscribe((servers) => {
          this.servers = servers.filter(
            (s) => this.runningOrHasName(s) && !this.alreadyExists(s)
          );
          this.servers.forEach((s) => (s.selected = true));

          this.loading = false;
        });
    }
  };

  alreadyExists = (hi: HostInformation): boolean => {
    return (
      this.existing_servers.find((e) => e.ipaddress === hi.ipaddress) !==
      undefined
    );
  };

  runningOrHasName = (hi: HostInformation): boolean => {
    return hi.is_running || hi.dnsname.length > 0;
  };

  onClickSaveServers = () => {
    const serversToSave: Server[] = [];
    for (let i = 0; i < this.servers.length; i++) {
      const server = this.servers[i];

      if (server.selected) {
        serversToSave.push(
          new Server(server.ipaddress, '', server.dnsname, [])
        );
      }
    }
    this.serverService.saveServers(serversToSave);
    this.ref.close();
  };

  onClickDeselectServer = (index: number) => {
    this.servers[index].selected = !this.servers[index].selected;
  };
}
