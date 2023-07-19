import { Component, OnDestroy, OnInit, inject } from '@angular/core';
import { FormControl, Validators } from '@angular/forms';
import { Store } from '@ngrx/store';
import { RxwebValidators, IpVersion } from '@rxweb/reactive-form-validators';
import { Observable, map, of } from 'rxjs';
import { Plugin } from 'src/app/services/plugins/types';
import { Feature, Server, ServerFeature } from 'src/app/services/servers/types';
import { SubscriptionHandler } from 'src/app/shared/subscriptionHandler';
import { selectAllPlugins } from 'src/app/state/plugin/plugin.selectors';
import {
  addServerFeature,
  saveServer,
} from 'src/app/state/server/server.actions';
import { selectAllServers } from 'src/app/state/server/server.selectors';

@Component({
  selector: 'app-add-server-modal',
  templateUrl: './add-server-modal.component.html',
  styleUrls: ['./add-server-modal.component.scss'],
})
export class AddServerModalComponent implements OnInit, OnDestroy {
  private store = inject(Store);

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

  servers$?: Observable<Server[]>;
  plugins$?: Observable<Plugin[]>;

  availablePlugins$?: Observable<Plugin[]>;
  currentFeatures: Feature[] = [];

  subscriptionHandler = new SubscriptionHandler(this);

  ngOnInit(): void {
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
      this.store.dispatch(
        saveServer({
          server: new Server(
            this.ipaddress.value,
            this.name.value ? this.name.value : ''
          ),
        })
      );
    }
  };

  addFeatureToServer = () => {
    if (this.selectedServer && this.selectedPlugin && this.selectedServer) {
      const feature = new Feature(
        this.selectedPlugin.id,
        this.selectedPlugin.name,
        [],
        []
      );

      const serverFeature = new ServerFeature(this.selectedServer.ipaddress, [
        feature,
      ]);

      this.store.dispatch(addServerFeature({ serverFeature: serverFeature }));

      this.selectedPlugin = undefined;
      this.selectedServer = undefined;
    }
  };

  onChangeServer = () => {
    this.currentFeatures = this.selectedServer
      ? this.selectedServer.features
      : [];

    if (this.plugins$) {
      this.availablePlugins$ = this.plugins$.pipe(
        map((plugins) =>
          plugins.filter(
            (p) => !this.currentFeatures.find((f) => f.id === p.id)
          )
        )
      );
    }
  };
}
