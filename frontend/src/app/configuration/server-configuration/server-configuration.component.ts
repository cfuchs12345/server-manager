import { Component } from '@angular/core';
import { AutoDiscoveryDialog, dialogSettings as autodiscoverDialogSettings } from './dialogs/dialog-autodiscover'
import { FeatureScanDialog , dialogSettings as featurescanDialogSettings } from './dialogs/dialog-feature-scan'
import { AddServerDialog , dialogSettings as addServerDialogSettings } from './dialogs/dialog-add-server'
import { DeleteServerDialog , dialogSettings as deleteDialogSettings } from './dialogs/dialog-delete-server'
import { ConfigureFeaturesDialog, dialogSettings as configureFeaturesDialogSettings } from './dialogs/dialog-configure-features';
import { MatDialog } from '@angular/material/dialog';

@Component({
  selector: 'app-server-configuration',
  templateUrl: './server-configuration.component.html',
  styleUrls: ['./server-configuration.component.scss']
})
export class ServerConfigurationComponent {
  title: string = 'Server Configuration';
  description: string = 'Configure the list of available/known servers.'

  buttonTextAutoDiscover: string = 'Autodiscovery';
  buttonTextAddManually: string = 'Add Server/Feature'
  buttonTextDelete: string = "Delete Server/Feature";
  buttonTextFeatureScan: string = 'Feature Scan';
  buttonTextConfigureFeatures: string = 'Configure Features';

  constructor (private dialog: MatDialog) {}

  openDialogAutodiscovery = () => {
    this.dialog.open(AutoDiscoveryDialog, {
      ...autodiscoverDialogSettings()
    });
  }

  openDialogFeatureScan = () => {
    this.dialog.open(FeatureScanDialog, {
      ...featurescanDialogSettings()
    });
  }

  openDialogAddManually = () => {
    this.dialog.open(AddServerDialog, {
      ...addServerDialogSettings()
    });
  }

  openDialogDelete = () => {
    this.dialog.open(DeleteServerDialog, {
      ...deleteDialogSettings()
    });
  }

  openDialogConfigureFeatures = () => {
    this.dialog.open(ConfigureFeaturesDialog, {
      ...configureFeaturesDialogSettings()
    });
  }
}



