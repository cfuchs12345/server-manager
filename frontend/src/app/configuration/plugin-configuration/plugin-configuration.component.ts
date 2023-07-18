import { Component , inject} from '@angular/core';
import { MatDialog } from '@angular/material/dialog';
import {
  DisablePluginsDialog,
  dialogSettings as managePluginDialogSettings,
} from './dialogs/dialog-manageplugins';
import {
  ListPluginsDialog,
  dialogSettings as listPluginDialogSettings,
} from './dialogs/dialog-listplugins';

@Component({
  selector: 'app-plugin-configuration',
  templateUrl: './plugin-configuration.component.html',
  styleUrls: ['./plugin-configuration.component.scss'],
})
export class PluginConfigurationComponent {
  private dialog = inject(MatDialog);

  title = 'Plugin Configuration';
  description = 'Configure Plugins.';

  buttonTextManagePlugins = 'Manage Plugins';
  buttonTextListPlugins = 'List Plugins';

  openDialogManagePlugins() {
    this.dialog.open(DisablePluginsDialog, {
      ...managePluginDialogSettings(),
    });
  }

  openDialogListPlugins() {
    this.dialog.open(ListPluginsDialog, {
      ...listPluginDialogSettings(),
    });
  }
}
