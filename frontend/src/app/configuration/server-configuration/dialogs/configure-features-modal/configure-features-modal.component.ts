import { Component, OnDestroy, OnInit } from '@angular/core';
import {
  FormBuilder,
  FormControl,
  FormGroup,
  ValidatorFn,
  Validators,
} from '@angular/forms';
import { error } from 'console';
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

  showPasswordCredentials: Map<String, boolean> = new Map();
  passwordCredentials: Map<String, boolean> = new Map();

  form: FormGroup;

  selectedServer: Server | undefined = undefined;
  selectedServerFullData: Server | undefined = undefined;
  selectedFeature: Feature | undefined = undefined;

  paramsFromPlugin: ParamDefinition[] = [];
  paramsFromFeature: Param[] = [];

  credentialsFromPlugin: CredentialDefinition[] = [];
  credentialFromFeature: Credential[] = [];

  servers: Server[] = [];
  features: Feature[] = [];
  plugins: Plugin[] = [];

  subscriptionServers: Subscription | undefined = undefined;
  subscriptionServerFullData: Subscription | undefined = undefined;
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
        this.plugins = plugins;
      }
    );
    this.pluginService.loadPlugins();
  }

  ngOnDestroy(): void {
    if (this.subscriptionServers) {
      this.subscriptionServers.unsubscribe();
    }
    if (this.subscriptionPlugins) {
      this.subscriptionPlugins.unsubscribe();
    }
    if (this.subscriptionServerFullData) {
      this.subscriptionServerFullData.unsubscribe();
    }
  }

  getCurrentParamValue = (name: string): string => {
    if (!this.paramsFromFeature) {
      return '';
    }

    const param = this.paramsFromFeature.find((param) => param.name === name);
    if (param) {
      return param.value;
    }
    return '';
  };

  createInputControls = () => {
    this.paramsFromPlugin.forEach((param) =>
      this.form.addControl(
        'param.' + param.name,
        new FormControl('', param.mandatory ? [Validators.required] : [])
      )
    );
    this.credentialsFromPlugin.forEach((credential) => {
      if (credential.encrypt) {
        this.passwordCredentials.set(credential.name, true);
      }
      this.form.addControl(
        'credential.' + credential.name,
        new FormControl('', credential.mandatory ? [Validators.required] : [])
      );
    });
  };

  setInitialValuesOnInputControls = () => {
    this.paramsFromFeature.forEach((param) =>
      this.form.controls[`param.${param.name}`].setValue(param.value)
    );
    this.credentialFromFeature.forEach((credential) =>
      this.form.controls[`credential.${credential.name}`].setValue(
        credential.value
      )
    );
  };

  getSelectedPlugin = (plugins: Plugin[]): Plugin | undefined => {
    const plugin = plugins.find(
      (plugin) => plugin.id === this.selectedFeature?.id
    );
    return plugin;
  };

  getDefaultParamValue = (name: string): string => {
    if (!this.paramsFromPlugin) {
      return '';
    }

    const paramDef = this.paramsFromPlugin.find((param) => param.name === name);
    if (
      paramDef &&
      paramDef.default_value &&
      paramDef.default_value.length > 0
    ) {
      return paramDef.default_value;
    }
    return 'None';
  };

  getCurrentCredentialValue = (name: string): string => {
    if (!this.credentialFromFeature) {
      return '';
    }

    const credential = this.credentialFromFeature.find(
      (credential) => credential.name === name
    );
    if (credential) {
      return credential.value;
    }
    return '';
  };

  getDefaultCredentialValue = (name: string): string => {
    if (!this.credentialsFromPlugin) {
      return '';
    }

    const credential = this.credentialsFromPlugin.find(
      (credential) => credential.name === name
    );
    if (
      credential &&
      credential.default_value &&
      credential.default_value.length > 0
    ) {
      return credential.default_value;
    }
    return 'None';
  };

  isPasswordCredential = (name: string): boolean => {
    const res = this.passwordCredentials.get(name);

    return res === true;
  };

  isShowPasswordCredential = (name: string): boolean => {
    const res = this.showPasswordCredentials.get(name);

    return res === true;
  };

  onClickShowPasswordCredential = (name: string) => {
    if (!this.showPasswordCredentials.get(name)) {
      this.showPasswordCredentials.set(name, true);
    } else {
      this.showPasswordCredentials.set(name, false);
    }
  };

  onClickSaveFeatureSettings = () => {
    const selectedFeature = this.selectedFeature;

    if (
      !this.selectedServerFullData ||
      !selectedFeature ||
      !this.selectedServerFullData.ipaddress
    ) {
      return;
    }
    const feature = this.selectedServerFullData.features.find(
      (feature) => feature.id === selectedFeature.id
    );
    if (!feature) {
      return;
    }

    feature.credentials = this.makeCredentials();
    feature.params = this.makeParams();

    this.serverService.updateServer(this.selectedServerFullData);
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
    const ref = this;

    this.form = this.formBuilder.group({});
    if (this.selectedServer?.ipaddress) {
      this.subscriptionServerFullData = this.serverService
        .getServer(this.selectedServer?.ipaddress, true)
        .subscribe({
          next: (server) => {
            this.selectedServerFullData = server;
            ref.features = ref.selectedServerFullData
              ? ref.selectedServerFullData.features
              : [];
            ref.resetSelections();
            setTimeout(ref.createInputControls, 0);
          },
          error: (err) => {},
          complete: () => {},
        });
    }
  };

  onChangeFeature = () => {
    this.form = this.formBuilder.group({});

    if (this.selectedFeature) {
      if (this.plugins) {
        const shownPlugin = this.getSelectedPlugin(this.plugins);

        if (shownPlugin) {
          this.paramsFromPlugin = shownPlugin.params;
          this.credentialsFromPlugin = shownPlugin.credentials;

          this.paramsFromFeature = this.selectedFeature.params;
          this.credentialFromFeature = this.selectedFeature.credentials;

          this.createInputControls();
          this.setInitialValuesOnInputControls();
        }
      } else {
        this.resetPluginSelections();
      }
    } else {
      this.resetFeatureSelections();
    }
  };

  private resetFeatureSelections() {
    this.paramsFromFeature = [];
    this.credentialFromFeature = [];
  }

  private resetPluginSelections() {
    this.paramsFromPlugin = [];
    this.credentialsFromPlugin = [];
  }

  private resetSelections() {
    this.resetFeatureSelections();
    this.resetPluginSelections();
    this.selectedFeature = undefined;
  }
}
