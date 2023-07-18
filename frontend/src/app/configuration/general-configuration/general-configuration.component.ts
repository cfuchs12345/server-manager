import { Component, inject } from '@angular/core';
import { MatDialog } from '@angular/material/dialog';
import {
  ConfigureDNSDialog as ConfigureDNSDialog,
  dialogSettings as configureDNSDialogSettings,
} from './dialogs/dialog-configure-dns';
import {
  ConfigureUsersDialog,
  dialogSettings as configureUsersDialogSettings,
} from './dialogs/dialog-configure-users';
import {
  ChangePasswordDialog,
  dialogSettings as changePasswordDialogSettings,
} from './dialogs/dialog-change-password';
import {
  ConfigImExportDialog,
  dialogSettings as changeDialogImExportConfig,
} from './dialogs/dialog-config-im-and-export';

@Component({
  selector: 'app-general-configuration',
  templateUrl: './general-configuration.component.html',
  styleUrls: ['./general-configuration.component.scss'],
})
export class GeneralConfigurationComponent {
  private dialog = inject(MatDialog);

  title = 'General Configuration';
  description = 'Configure User Permissions and so on.';

  buttonTextManageDNSServers = 'DNS Servers';
  buttonTextManageUsers = 'Users';
  buttonTextChangePassword = 'Change your password';
  buttonTextImExportConfig = 'Import/Export of Config';

  openDialogManageDNSServers = () => {
    this.dialog.open(ConfigureDNSDialog, {
      ...configureDNSDialogSettings(),
    });
  };

  openDialogManageUsers = () => {
    this.dialog.open(ConfigureUsersDialog, {
      ...configureUsersDialogSettings(),
    });
  };

  openDialogChangePassword = () => {
    this.dialog.open(ChangePasswordDialog, {
      ...changePasswordDialogSettings(),
    });
  };

  openDialogImExportConfig = () => {
    this.dialog.open(ConfigImExportDialog, {
      ...changeDialogImExportConfig(),
    });
  };
}
