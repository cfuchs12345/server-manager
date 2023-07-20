import { Component } from '@angular/core';
import { GeneralConfigurationComponent } from '../general-configuration/general-configuration.component';
import { PluginConfigurationComponent } from '../plugin-configuration/plugin-configuration.component';
import { ServerConfigurationComponent } from '../server-configuration/server-configuration.component';
import { ConfigurationGroupComponent } from '../configuration-group/configuration-group.component';

@Component({
    selector: 'app-configuration',
    templateUrl: './configuration.component.html',
    styleUrls: ['./configuration.component.scss'],
    standalone: true,
    imports: [
        ConfigurationGroupComponent,
        ServerConfigurationComponent,
        PluginConfigurationComponent,
        GeneralConfigurationComponent,
    ],
})
export class ConfigurationComponent {}
