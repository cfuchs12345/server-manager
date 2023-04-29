import { Component, OnInit } from '@angular/core';
import { FormControl, Validators } from '@angular/forms';
import { MatDialog } from '@angular/material/dialog';
import { Subscription } from 'rxjs';
import { PluginService } from 'src/app/services/plugins/plugin.service';
import { Plugin } from 'src/app/services/plugins/types';
import { ServerService } from 'src/app/services/servers/server.service';
import { Feature, Server, ServerFeature } from 'src/app/services/servers/types';
import { ConfirmDialogComponent } from 'src/app/ui/confirm-dialog/confirm-dialog.component';
import { ServerAddressType } from 'src/types/ServerAddress';

@Component({
  selector: 'app-add-server-modal',
  templateUrl: './add-server-modal.component.html',
  styleUrls: ['./add-server-modal.component.scss'],
})
export class AddServerModalComponent implements OnInit {
  ipplaceholder: string = 'xxx.xxx.xxx.xxx';
  ipAddressLabel: string = 'IP Address';
  ipaddressHint: string = 'Example: 192.168.178.111';
  nameLabel: string = 'Name';
  nameHint: string = '';
  buttonTextAddServer: string = 'Add Server';
  name = new FormControl('', []);
  ipaddress = new FormControl('', [
    Validators.required,
    Validators.pattern(ServerAddressType.IPV4),
  ]);

  buttonTextAddFeature="Add Feature";


  selectedServer: Server | undefined = undefined;
  selectedPlugin: Plugin | undefined = undefined;

  servers: Server[] = [];
  plugins: Plugin[] = [];
  availablePlugins: Plugin[] = [];
  currentFeatures: Feature[] = [];

  subscriptionServers: Subscription | undefined = undefined;
  subscriptionPlugins: Subscription | undefined = undefined;


  constructor(private serverService: ServerService, private pluginService: PluginService, private dialog: MatDialog) {
  }

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
    if (this.ipaddress.hasError('required')) {
      return 'You must enter a value';
    }
    return this.ipaddress.hasError('pattern')
      ? 'The IP address format is not correct'
      : 'Unknown error';
  }

  saveServer = () => {
    if (this.ipaddress.value && this.name.value) {
      this.serverService.saveServers([
        new Server(this.ipaddress.value, this.name.value),
      ]);
    }
  };

  addFeatureToServer = () => {
    if( this.selectedPlugin && this.selectedServer ) {
      const features = this.selectedServer.features;
      features.push(new Feature(this.selectedPlugin.id, this.selectedPlugin.name, [], []));
      const featureOfServer = new ServerFeature( this.selectedServer.ipaddress, features, true);
      this.serverService.updateServerFeatures([featureOfServer]);
    }
  }


  onChangeServer = () => {
    this.currentFeatures = this.selectedServer ? this.selectedServer.features : [];
    this.availablePlugins = this.plugins.filter((plugin) => !this.isFeatureAlreadySet(plugin.id, this.currentFeatures));
  };

  onChangeFeature = () => {
  };

  private isFeatureAlreadySet(id: string, features: Feature[]) {
    return features.filter( (feature) => feature.id === id).length > 0;
  }
}
