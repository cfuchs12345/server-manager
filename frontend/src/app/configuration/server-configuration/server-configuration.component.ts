import { Component, inject } from '@angular/core';
import {
  AutoDiscoveryDialogComponent,
  dialogSettings as autodiscoverDialogSettings,
} from './dialogs/dialog-autodiscover';
import {
  FeatureScanDialog,
  dialogSettings as featurescanDialogSettings,
} from './dialogs/dialog-feature-scan';
import {
  AddServerDialogComponent,
  dialogSettings as addServerDialogSettings,
} from './dialogs/dialog-add-server';
import {
  DeleteServerDialogComponent,
  dialogSettings as deleteDialogSettings,
} from './dialogs/dialog-delete-server';
import {
  ConfigureFeaturesDialogComponent,
  dialogSettings as configureFeaturesDialogSettings,
} from './dialogs/dialog-configure-features';
import { MatDialog } from '@angular/material/dialog';
import { MatButtonModule } from '@angular/material/button';
import { FlexModule } from '@angular/flex-layout/flex';
import { ConfigurationGroupComponent } from '../configuration-group/configuration-group.component';

@Component({
    selector: 'app-server-configuration',
    templateUrl: './server-configuration.component.html',
    styleUrls: ['./server-configuration.component.scss'],
    standalone: true,
    imports: [
        ConfigurationGroupComponent,
        FlexModule,
        MatButtonModule,
    ],
})
export class ServerConfigurationComponent {
  private dialog = inject(MatDialog);

  title = 'Server Configuration';
  description = 'Configure the list of available/known servers.';

  buttonTextAutoDiscover = 'Autodiscovery';
  buttonTextAddManually = 'Add Server/Feature';
  buttonTextDelete = 'Delete Server/Feature';
  buttonTextFeatureScan = 'Feature Scan';
  buttonTextConfigureFeatures = 'Configure Features';


  openDialogAutodiscovery = () => {
    this.dialog.open(AutoDiscoveryDialogComponent, {
      ...autodiscoverDialogSettings(),
    });
  };

  openDialogFeatureScan = () => {
    this.dialog.open(FeatureScanDialog, {
      ...featurescanDialogSettings(),
    });
  };

  openDialogAddManually = () => {
    this.dialog.open(AddServerDialogComponent, {
      ...addServerDialogSettings(),
    });
  };

  openDialogDelete = () => {
    this.dialog.open(DeleteServerDialogComponent, {
      ...deleteDialogSettings(),
    });
  };

  openDialogConfigureFeatures = () => {
    this.dialog.open(ConfigureFeaturesDialogComponent, {
      ...configureFeaturesDialogSettings(),
    });
  };
}
