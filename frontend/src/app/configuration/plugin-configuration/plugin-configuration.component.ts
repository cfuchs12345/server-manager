import { Component } from '@angular/core';
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
  title: string = 'Plugin Configuration';
  description: string = 'Configure Plugins.';

  buttonTextManagePlugins: string = 'Manage Plugins';
  buttonTextListPlugins: string = 'List Plugins';

  constructor(private dialog: MatDialog) {}

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
