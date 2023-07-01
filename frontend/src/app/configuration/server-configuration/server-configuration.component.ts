import { Component } from '@angular/core';
import {
  AutoDiscoveryDialog,
  dialogSettings as autodiscoverDialogSettings,
} from './dialogs/dialog-autodiscover';
import {
  FeatureScanDialog,
  dialogSettings as featurescanDialogSettings,
} from './dialogs/dialog-feature-scan';
import {
  AddServerDialog,
  dialogSettings as addServerDialogSettings,
} from './dialogs/dialog-add-server';
import {
  DeleteServerDialog,
  dialogSettings as deleteDialogSettings,
} from './dialogs/dialog-delete-server';
import {
  ConfigureFeaturesDialog,
  dialogSettings as configureFeaturesDialogSettings,
} from './dialogs/dialog-configure-features';
import { MatDialog } from '@angular/material/dialog';

@Component({
  selector: 'app-server-configuration',
  templateUrl: './server-configuration.component.html',
  styleUrls: ['./server-configuration.component.scss'],
})
export class ServerConfigurationComponent {
  title = 'Server Configuration';
  description = 'Configure the list of available/known servers.';

  buttonTextAutoDiscover = 'Autodiscovery';
  buttonTextAddManually = 'Add Server/Feature';
  buttonTextDelete = 'Delete Server/Feature';
  buttonTextFeatureScan = 'Feature Scan';
  buttonTextConfigureFeatures = 'Configure Features';

  constructor(private dialog: MatDialog) {}

  openDialogAutodiscovery = () => {
    this.dialog.open(AutoDiscoveryDialog, {
      ...autodiscoverDialogSettings(),
    });
  };

  openDialogFeatureScan = () => {
    this.dialog.open(FeatureScanDialog, {
      ...featurescanDialogSettings(),
    });
  };

  openDialogAddManually = () => {
    this.dialog.open(AddServerDialog, {
      ...addServerDialogSettings(),
    });
  };

  openDialogDelete = () => {
    this.dialog.open(DeleteServerDialog, {
      ...deleteDialogSettings(),
    });
  };

  openDialogConfigureFeatures = () => {
    this.dialog.open(ConfigureFeaturesDialog, {
      ...configureFeaturesDialogSettings(),
    });
  };
}
