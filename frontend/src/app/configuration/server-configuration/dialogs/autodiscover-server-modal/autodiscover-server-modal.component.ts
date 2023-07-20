import { Component, OnDestroy, OnInit, inject } from '@angular/core';
import { FormControl, Validators, FormsModule, ReactiveFormsModule } from '@angular/forms';
import { HostInformation, Server } from '../../../../services/servers/types';
import { MatDialogRef } from '@angular/material/dialog';
import { AutoDiscoveryDialogComponent } from '../dialog-autodiscover';
import { Observable } from 'rxjs';
import { ServerDiscoveryService } from 'src/app/services/servers/server-discovery.service';
import { GeneralService } from 'src/app/services/general/general.service';
import { DNSServer } from 'src/app/services/general/types';
import { ErrorService, Source } from 'src/app/services/errors/error.service';
import { RxwebValidators, IpVersion } from '@rxweb/reactive-form-validators';
import { Store } from '@ngrx/store';
import { selectAllServers } from 'src/app/state/server/server.selectors';
import { SubscriptionHandler } from 'src/app/shared/subscriptionHandler';
import { saveServers } from 'src/app/state/server/server.actions';
import { MatTableModule } from '@angular/material/table';
import { MatButtonModule } from '@angular/material/button';
import { NgIf } from '@angular/common';
import { MatInputModule } from '@angular/material/input';
import { MatFormFieldModule } from '@angular/material/form-field';
import { FlexModule } from '@angular/flex-layout/flex';

@Component({
    selector: 'app-autodiscover-server-modal',
    templateUrl: './autodiscover-server-modal.component.html',
    styleUrls: ['./autodiscover-server-modal.component.scss'],
    standalone: true,
    imports: [
        FlexModule,
        MatFormFieldModule,
        MatInputModule,
        FormsModule,
        ReactiveFormsModule,
        NgIf,
        MatButtonModule,
        MatTableModule,
    ],
})
export class AutodiscoverServerModalComponent implements OnInit, OnDestroy {
  private store = inject(Store);
  private discoverService = inject(ServerDiscoveryService);
  private generalService = inject(GeneralService);
  private errorService = inject(ErrorService);
  private ref = inject(MatDialogRef<AutoDiscoveryDialogComponent>);

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

  servers$?: Observable<Server[]>;

  subscriptionHandler = new SubscriptionHandler(this);

  ngOnInit(): void {
    this.servers$ = this.store.select(selectAllServers);

    this.loadDNSServers();

    this.servers$.subscribe((servers) => {
      this.existing_servers = servers;
    });
  }

  private loadDNSServers() {
    this.generalService.listDNSServers(this.setDNSServers);
  }

  private setDNSServers = (dnsservers: DNSServer[]) => {
    if (dnsservers) {
      this.dnsservers = dnsservers;
    }
  };
  // doesn't seem to work when written as arrow function!?
  ngOnDestroy(): void {
    this.subscriptionHandler.onDestroy();
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
      this.subscriptionHandler.subscription = this.discoverService
        .autoDiscoverServers(value, true)
        .subscribe((servers) => {
          this.servers = servers
            .filter((s) => this.runningOrHasName(s) && !this.alreadyExists(s))
            .map((s) => {
              s.selected = true;
              return s;
            });

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
    const servers = this.servers
      .filter((s) => s.selected)
      .map((s) => new Server(s.ipaddress, '', s.dnsname, []));
    this.store.dispatch(saveServers({ servers }));
    this.ref.close();
  };

  onClickDeselectServer = (index: number) => {
    this.servers[index].selected = !this.servers[index].selected;
  };
}
