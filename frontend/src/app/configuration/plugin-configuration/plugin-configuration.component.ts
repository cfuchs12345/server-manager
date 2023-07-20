import { Component , inject} from '@angular/core';
import { MatDialog } from '@angular/material/dialog';
import {
  DisablePluginsDialogComponent,
  dialogSettings as managePluginDialogSettings,
} from './dialogs/dialog-manageplugins';
import {
  ListPluginsDialogComponent,
  dialogSettings as listPluginDialogSettings,
} from './dialogs/dialog-listplugins';
import { MatButtonModule } from '@angular/material/button';
import { FlexModule } from '@angular/flex-layout/flex';
import { ConfigurationGroupComponent } from '../configuration-group/configuration-group.component';

@Component({
    selector: 'app-plugin-configuration',
    templateUrl: './plugin-configuration.component.html',
    styleUrls: ['./plugin-configuration.component.scss'],
    standalone: true,
    imports: [
        ConfigurationGroupComponent,
        FlexModule,
        MatButtonModule,
    ],
})
export class PluginConfigurationComponent {
  private dialog = inject(MatDialog);

  title = 'Plugin Configuration';
  description = 'Configure Plugins.';

  buttonTextManagePlugins = 'Manage Plugins';
  buttonTextListPlugins = 'List Plugins';

  openDialogManagePlugins() {
    this.dialog.open(DisablePluginsDialogComponent, {
      ...managePluginDialogSettings(),
    });
  }

  openDialogListPlugins() {
    this.dialog.open(ListPluginsDialogComponent, {
      ...listPluginDialogSettings(),
    });
  }
}
