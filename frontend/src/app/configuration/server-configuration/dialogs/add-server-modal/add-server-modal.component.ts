import { Component, OnInit } from '@angular/core';
import {
  AbstractControl,
  FormControl,
  ValidationErrors,
  ValidatorFn,
  Validators,
} from '@angular/forms';
import { RxwebValidators, IpVersion } from '@rxweb/reactive-form-validators';
import { Subscription } from 'rxjs';
import { PluginService } from 'src/app/services/plugins/plugin.service';
import { Plugin } from 'src/app/services/plugins/types';
import { ServerService } from 'src/app/services/servers/server.service';
import { Feature, Server, ServerFeature } from 'src/app/services/servers/types';

@Component({
  selector: 'app-add-server-modal',
  templateUrl: './add-server-modal.component.html',
  styleUrls: ['./add-server-modal.component.scss'],
})
export class AddServerModalComponent implements OnInit {
  ipplaceholder: string = 'xxx.xxx.xxx.xxx or xxxx:xxxx...';
  ipAddressLabel: string = 'IP Address';
  ipaddressHint: string = 'Example: 192.168.178.111 or FE80::1';
  nameLabel: string = 'Name';
  nameHint: string = '';
  buttonTextAddServer: string = 'Add Server';
  name = new FormControl('', []);
  ipaddress = new FormControl('', [
    Validators.required,
    RxwebValidators.ip({ version: IpVersion.AnyOne }),
  ]);

  buttonTextAddFeature = 'Add Feature';

  selectedServer: Server | undefined = undefined;
  selectedPlugin: Plugin | undefined = undefined;

  servers: Server[] = [];
  plugins: Plugin[] = [];
  availablePlugins: Plugin[] = [];
  currentFeatures: Feature[] = [];

  subscriptionServers: Subscription | undefined = undefined;
  subscriptionPlugins: Subscription | undefined = undefined;

  constructor(
    private serverService: ServerService,
    private pluginService: PluginService
  ) {}

  ngOnInit(): void {
    this.subscriptionServers = this.serverService.servers.subscribe(
      (servers) => {
        if (servers) {
          this.servers = servers;
        } else {
          // clear messages when empty message received
          this.servers = [];
        }
      }
    );

    this.subscriptionPlugins = this.pluginService.plugins.subscribe(
      (plugins) => {
        this.plugins = plugins;
      }
    );

    this.serverService.listServers();
    this.pluginService.loadPlugins();
  }

  getIPAddressErrorMessage() {
    console.log(this.ipaddress.errors);
    if (this.ipaddress.hasError('required')) {
      return 'You must enter a value';
    }
    return this.ipaddress.hasError('ip')
      ? 'The IP address format is not correct'
      : 'Unknown error';
  }

  saveServer = () => {
    if (this.ipaddress.value && this.name.value) {
      this.serverService.saveServers([
        new Server(this.ipaddress.value, this.name.value),
      ]);
      this.serverService.listServers(); // this refreshes also the server list on the main screen
    }
  };

  addFeatureToServer = () => {
    const ref = this;
    if (this.selectedServer) {
      this.serverService
        .getServer(this.selectedServer.ipaddress, true)
        .subscribe({
          next: (server) => {
            if (ref.selectedPlugin && ref.selectedServer) {
              const features = server.features;
              features.push(
                new Feature(
                  ref.selectedPlugin.id,
                  ref.selectedPlugin.name,
                  [],
                  []
                )
              );
              const featureOfServer = new ServerFeature(
                ref.selectedServer.ipaddress,
                features,
                true
              );
              this.serverService.updateServer(server);
            }
          },
          error: (err) => {},
          complete: () => {
            this.serverService.listServers();
          },
        });
    }
  };

  onChangeServer = () => {
    this.currentFeatures = this.selectedServer
      ? this.selectedServer.features
      : [];
    this.availablePlugins = this.plugins.filter(
      (plugin) => !this.isFeatureAlreadySet(plugin.id, this.currentFeatures)
    );
  };

  onChangeFeature = () => {};

  private isFeatureAlreadySet(id: string, features: Feature[]) {
    return features.filter((feature) => feature.id === id).length > 0;
  }
}
