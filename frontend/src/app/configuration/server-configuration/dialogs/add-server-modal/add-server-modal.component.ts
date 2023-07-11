import { Component, OnDestroy } from '@angular/core';
import { FormControl, Validators } from '@angular/forms';
import { Store } from '@ngrx/store';
import { RxwebValidators, IpVersion } from '@rxweb/reactive-form-validators';
import { Observable,  map, of } from 'rxjs';
import { ErrorService } from 'src/app/services/errors/error.service';
import { Plugin } from 'src/app/services/plugins/types';
import { ServerService } from 'src/app/services/servers/server.service';
import { Feature, Server } from 'src/app/services/servers/types';
import { SubscriptionHandler } from 'src/app/shared/subscriptionHandler';
import { selectAllPlugins } from 'src/app/state/plugin/plugin.selectors';
import { selectAllServers } from 'src/app/state/server/server.selectors';

@Component({
  selector: 'app-add-server-modal',
  templateUrl: './add-server-modal.component.html',
  styleUrls: ['./add-server-modal.component.scss'],
})
export class AddServerModalComponent implements OnDestroy {
  ipPlaceholder = 'xxx.xxx.xxx.xxx or xxxx:xxxx...';
  ipAddressLabel = 'IP Address';
  ipaddressHint = 'Example: 192.168.178.111 or FE80::1';
  nameLabel = 'Name';
  nameHint = '';
  buttonTextAddServer = 'Add Server';
  name = new FormControl('', []);
  ipaddress = new FormControl('', [
    Validators.required,
    RxwebValidators.ip({ version: IpVersion.AnyOne }),
  ]);

  buttonTextAddFeature = 'Add Feature';

  selectedServer: Server | undefined = undefined;
  selectedPlugin: Plugin | undefined = undefined;

  servers$: Observable<Server[]>;
  plugins$: Observable<Plugin[]>;

  availablePlugins$: Observable<Plugin[]>;
  currentFeatures: Feature[] = [];

  subscriptionHandler = new SubscriptionHandler(this);

  constructor(
    private store: Store,
    private serverService: ServerService,
    private errorService: ErrorService
  ) {
    this.servers$ = this.store.select(selectAllServers);
    this.plugins$ = this.store.select(selectAllPlugins);
    this.availablePlugins$ = of();
  }

  ngOnDestroy(): void {
    this.subscriptionHandler.onDestroy();
  }

  getIPAddressErrorMessage() {
    if (this.ipaddress.hasError('required')) {
      return 'You must enter a value';
    }
    return this.ipaddress.hasError('ip')
      ? 'The IP address format is not correct'
      : 'Unknown error';
  }

  saveServer = () => {
    if (this.ipaddress.value) {
      this.serverService.saveServers([
        new Server(this.ipaddress.value, this.name.value ? this.name.value: ''),
      ]);
    }
  };

  addFeatureToServer = () => {
    if (this.selectedServer) {
      this.subscriptionHandler.subscription = this.serverService
        .getServer(this.selectedServer.ipaddress, true)
        .subscribe({
          next: (server) => {
            if (this.selectedPlugin && this.selectedServer) {
              const features = server.features;
              features.push(
                new Feature(
                  this.selectedPlugin.id,
                  this.selectedPlugin.name,
                  [],
                  []
                )
              );
              this.serverService.updateServer(server);

              this.selectedPlugin = undefined;
              this.selectedServer = undefined;
            }
          },
        });
    }
  };

  onChangeServer = () => {
    this.currentFeatures = this.selectedServer
      ? this.selectedServer.features
      : [];

    this.availablePlugins$ = this.plugins$.pipe(
      map((plugins) =>
        plugins.filter((p) => !this.currentFeatures.find((f) => f.id === p.id))
      )
    );
  };
}
