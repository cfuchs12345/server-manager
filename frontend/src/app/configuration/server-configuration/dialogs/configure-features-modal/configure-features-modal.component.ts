import { Component, OnDestroy, OnInit } from '@angular/core';
import { FormBuilder, FormControl, FormGroup } from '@angular/forms';
import { Subscription } from 'rxjs';
import { PluginService } from 'src/app/services/plugins/plugin.service';
import {
  CredentialDefinition,
  ParamDefinition,
  Plugin,
} from 'src/app/services/plugins/types';
import { ServerService } from 'src/app/services/servers/server.service';
import {
  Credential,
  Feature,
  Param,
  Server,
  ServerFeature,
} from 'src/app/services/servers/types';

@Component({
  selector: 'app-configure-features-modal',
  templateUrl: './configure-features-modal.component.html',
  styleUrls: ['./configure-features-modal.component.scss'],
})
export class ConfigureFeaturesModalComponent implements OnInit, OnDestroy {
  buttonTextSaveFeatureSettings = 'Save Feature Settings';

  form: FormGroup;

  selectedServer: Server | undefined = undefined;
  selectedFeature: Feature | undefined = undefined;

  paramsFromPlugin: ParamDefinition[] = [];
  paramsFromFeature: Param[] = [];

  credentialsFromPlugin: CredentialDefinition[] = [];
  credentialFromFeature: Credential[] = [];

  servers: Server[] = [];
  features: Feature[] = [];

  subscriptionServers: Subscription | undefined = undefined;
  subscriptionPlugins: Subscription | undefined = undefined;

  constructor(
    private serverService: ServerService,
    private pluginService: PluginService,
    private formBuilder: FormBuilder
  ) {
    this.form = formBuilder.group({});
  }

  ngOnInit(): void {
    this.subscriptionServers = this.serverService.servers.subscribe(
      (servers) => {
        if (servers) {
          this.servers = servers.filter(
            (server) => server.features && server.features.length > 0
          );
        } else {
          // clear messages when empty message received
          this.servers = [];
        }
      }
    );

    this.subscriptionPlugins = this.pluginService.plugins.subscribe(
      (plugins) => {
        // always reset form
        this.form = this.formBuilder.group({});

        if (plugins) {
          const shownPlugin = this.getSelectedPlugin(plugins);

          if (shownPlugin) {
            this.paramsFromPlugin = shownPlugin.params;
            this.credentialsFromPlugin = shownPlugin.credentials;

            this.createInputControls();
            this.setInitialValuesOnInputControls();
          }
        } else {
          this.paramsFromPlugin = [];
          this.credentialsFromPlugin = [];
        }
      }
    );
  }

  ngOnDestroy(): void {
    if (this.subscriptionServers) {
      this.subscriptionServers.unsubscribe();
    }
    if (this.subscriptionPlugins) {
      this.subscriptionPlugins.unsubscribe();
    }
  }

  getCurrentParamValue = (name: string): string => {
    if (!this.paramsFromFeature) {
      return '';
    }

    const filtered = this.paramsFromFeature.filter(
      (param) => param.name === name
    );
    if (filtered.length > 0) {
      return filtered[0].value;
    }
    return '';
  };

  createInputControls = () => {
    this.paramsFromPlugin.forEach((param) =>
      this.form.addControl('param.' + param.name, new FormControl('', []))
    );
    this.credentialsFromPlugin.forEach((credential) =>
      this.form.addControl(
        'credential.' + credential.name,
        new FormControl('', [])
      )
    );
  }

  setInitialValuesOnInputControls = () => {
    this.paramsFromFeature.forEach((param) =>
      this.form.controls['param.' + param.name].setValue(param.value)
    );
    this.credentialFromFeature.forEach((credential) =>
      this.form.controls['credential.' + credential.name].setValue(
        credential.value
      )
    );
  }

  getSelectedPlugin = (plugins: Plugin[]): Plugin | undefined => {
    const filteredPlugins = plugins.filter(
      (plugin) => plugin.id === this.selectedFeature?.id
    );
    return filteredPlugins.length == 1 ? filteredPlugins[0] : undefined;
  }

  getDefaultParamValue = (name: string): string => {
    if (!this.paramsFromPlugin) {
      return '';
    }

    const filtered = this.paramsFromPlugin.filter(
      (param) => param.name === name
    );
    if (filtered.length > 0) {
      return filtered[0].default_value;
    }
    return '';
  };

  getCurrentCredentialValue = (name: string): string => {
    if (!this.credentialFromFeature) {
      return '';
    }

    const filtered = this.credentialFromFeature.filter(
      (credential) => credential.name === name
    );
    if (filtered.length > 0) {
      return filtered[0].value;
    }
    return '';
  };

  getDefaultCredentialValue = (name: string): string => {
    if (!this.credentialsFromPlugin) {
      return '';
    }

    const filtered = this.credentialsFromPlugin.filter(
      (credential) => credential.name === name
    );
    if (filtered.length > 0) {
      return filtered[0].default_value;
    }
    return '';
  };

  onClickSaveFeatureSettings = () => {
    const selectedFeature = this.selectedFeature;
    const selectedServer = this.selectedServer;

    if (!selectedServer || !selectedFeature || !selectedServer.ipaddress) {
      return;
    }
    const filtered = selectedServer.features.filter(
      (feature) => feature.id === selectedFeature.id
    );
    if (filtered.length != 1) {
      return;
    }

    const feature: Feature = filtered[0];
    feature.credentials = this.makeCredentials();
    feature.params = this.makeParams();

    const serverFeature = new ServerFeature(
      selectedServer.ipaddress,
      selectedServer.features,
      true
    );

    this.serverService.updateServerFeatures([serverFeature]);
  };

  makeCredentials = (): Credential[] => {
    const credentials_from_form = this.getValuesFromForm('credential.');

    const credentials: Credential[] = [];
    this.credentialsFromPlugin.forEach((credential) => {
      const value = credentials_from_form.get(credential.name);

      if (value) {
        credentials.push(new Credential(credential.name, value, false));
      }
    });
    return credentials;
  };

  makeParams = (): Param[] => {
    const params_from_form = this.getValuesFromForm('param.');

    const params: Param[] = [];
    this.paramsFromPlugin.forEach((param) => {
      const value = params_from_form.get(param.name);

      if (value) {
        params.push(new Param(param.name, value));
      }
    });
    return params;
  };

  getValuesFromForm = (prefix: string): Map<string, string> => {
    const map = new Map();

    Object.keys(this.form.controls).forEach((key) => {
      var control = this.form.controls[key];

      if (key.startsWith(prefix)) {
        map.set(key.replace(prefix, ''), control.value);
      }
    });
    return map;
  };

  onChangeServer = () => {
    this.features = this.selectedServer ? this.selectedServer.features : [];
    this.paramsFromFeature = [];
    this.paramsFromPlugin = [];
  };

  onChangeFeature = () => {
    if (this.selectedFeature?.params) {
      this.paramsFromFeature = this.selectedFeature.params;
      this.credentialFromFeature = this.selectedFeature.credentials;

      this.pluginService.loadPlugins();
    }
  };
}
