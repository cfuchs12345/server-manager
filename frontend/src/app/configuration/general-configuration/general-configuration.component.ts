import { Component } from '@angular/core';
import { MatDialog } from '@angular/material/dialog';
import { ConfigureDNSDialog as ConfigureDNSDialog, dialogSettings as configureDNSDialogSettings } from './dialogs/dialog-configure-dns';
import { ConfigureUsersDialog,  dialogSettings as configureUsersDialogSettings } from './dialogs/dialog-configure-users';
import { ChangePasswordDialog,  dialogSettings as changePasswordDialogSettings } from './dialogs/dialog-change-password';
import { ConfigImExportDialog,  dialogSettings as changeDialogImExportConfig } from './dialogs/dialog-config-im-and-export';

@Component({
  selector: 'app-general-configuration',
  templateUrl: './general-configuration.component.html',
  styleUrls: ['./general-configuration.component.scss']
})
export class GeneralConfigurationComponent {
  title: string = 'General Configuration';
  description: string = 'Configure User Permissions and so on.'

  buttonTextManageDNSServers: string = 'DNS Servers';
  buttonTextManageUsers: string = 'Users';
  buttonTextChangePassword: string = 'Change your password';
  buttonTextImExportConfig: string = "Import/Export of Config"

  constructor(private dialog: MatDialog) {}

  openDialogManageDNSServers = () => {
    this.dialog.open(ConfigureDNSDialog, {
      ...configureDNSDialogSettings()
    });
  }

  openDialogManageUsers = () => {
    this.dialog.open(ConfigureUsersDialog, {
      ...configureUsersDialogSettings()
    });
  }

  openDialogChangePassword= () => {
    this.dialog.open(ChangePasswordDialog, {
      ...changePasswordDialogSettings()
    });
  }

  openDialogImExportConfig=() => {
    this.dialog.open(ConfigImExportDialog, {
      ...changeDialogImExportConfig()
    });
  }
}
