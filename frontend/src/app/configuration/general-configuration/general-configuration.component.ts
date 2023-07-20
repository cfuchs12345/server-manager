import { Component, inject } from '@angular/core';
import { MatDialog } from '@angular/material/dialog';
import {
  ConfigureDNSDialogComponent as ConfigureDNSDialogComponent,
  dialogSettings as configureDNSDialogSettings,
} from './dialogs/dialog-configure-dns';
import {
  ConfigureUsersDialogComponent,
  dialogSettings as configureUsersDialogSettings,
} from './dialogs/dialog-configure-users';
import {
  ChangePasswordDialogComponent,
  dialogSettings as changePasswordDialogSettings,
} from './dialogs/dialog-change-password';
import {
  ConfigImExportDialogComponent,
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
    this.dialog.open(ConfigureDNSDialogComponent, {
      ...configureDNSDialogSettings(),
    });
  };

  openDialogManageUsers = () => {
    this.dialog.open(ConfigureUsersDialogComponent, {
      ...configureUsersDialogSettings(),
    });
  };

  openDialogChangePassword = () => {
    this.dialog.open(ChangePasswordDialogComponent, {
      ...changePasswordDialogSettings(),
    });
  };

  openDialogImExportConfig = () => {
    this.dialog.open(ConfigImExportDialogComponent, {
      ...changeDialogImExportConfig(),
    });
  };
}
