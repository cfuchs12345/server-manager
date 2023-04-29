import { Component } from '@angular/core';
import { MatDialog } from '@angular/material/dialog';
import { ConfigureDNSDialog as ConfigureDNSDialog, dialogSettings as configureDNSDialogSettings } from './dialogs/dialog-configuredns';

@Component({
  selector: 'app-general-configuration',
  templateUrl: './general-configuration.component.html',
  styleUrls: ['./general-configuration.component.scss']
})
export class GeneralConfigurationComponent {
  title: string = 'General Configuration';
  description: string = 'Configure User Permissions and so on.'

  buttonTextManageDNSServers: string = 'DNS Servers';

  constructor(private dialog: MatDialog) {}

  openDialogManageDNSServers() {
    this.dialog.open(ConfigureDNSDialog, {
      ...configureDNSDialogSettings()
    });
  }
}
