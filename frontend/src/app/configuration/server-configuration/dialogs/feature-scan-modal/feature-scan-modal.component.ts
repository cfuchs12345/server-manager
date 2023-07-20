import { Component, OnDestroy, inject } from '@angular/core';
import { ServerFeature } from 'src/app/services/servers/types';
import { MatDialogRef } from '@angular/material/dialog';
import { ServerDiscoveryService } from 'src/app/services/servers/server-discovery.service';
import { SubscriptionHandler } from 'src/app/shared/subscriptionHandler';
import { Store } from '@ngrx/store';
import { addServerFeatures } from 'src/app/state/server/server.actions';
import { MatTableModule } from '@angular/material/table';
import { NgIf } from '@angular/common';
import { MatButtonModule } from '@angular/material/button';
import { FlexModule } from '@angular/flex-layout/flex';

@Component({
    selector: 'app-feature-scan-modal',
    templateUrl: './feature-scan-modal.component.html',
    styleUrls: ['./feature-scan-modal.component.scss'],
    standalone: true,
    imports: [
        FlexModule,
        MatButtonModule,
        NgIf,
        MatTableModule,
    ],
})
export class FeatureScanModalComponent implements OnDestroy {
  private discoveryService = inject(ServerDiscoveryService);
  private store = inject(Store);
  private ref = inject(MatDialogRef<FeatureScanModalComponent>);

  buttonTextScanFeature = 'Start';
  buttonTextWorking = 'Working...';
  buttonTextSaveServerFeatures = 'Save Features';

  displayedColumns = ['selected', 'ipaddress', 'features'];

  isWorking = false;

  discoveredServerFeatures: ServerFeature[] = [];
  subscriptionHandler = new SubscriptionHandler(this);

  ngOnDestroy(): void {
    this.subscriptionHandler.onDestroy();
  }

  private preSelectAllFeatures = (serverFeatures: ServerFeature[]) => {
    serverFeatures.forEach((f) => (f.selected = true));
  };

  featuresFound = (): boolean => {
    return this.discoveredServerFeatures.length > 0;
  };

  getFeaturesAsString = (feature: ServerFeature) => {
    return feature.features.map((feature) => feature.name).join(', ');
  };

  onClickScanFeature = () => {
    this.isWorking = true;
    this.subscriptionHandler.subscription = this.discoveryService
      .scanFeatureOfAllServers()
      .subscribe((serverFeatures) => {
        this.isWorking = false;
        this.preSelectAllFeatures(serverFeatures);

        this.discoveredServerFeatures = serverFeatures;
      });
  };

  onClickDeselectServer = (index: number) => {
    this.discoveredServerFeatures[index].selected =
      !this.discoveredServerFeatures[index].selected;
  };

  onClickSaveServerFeatures = () => {
    const serverFeatures = this.discoveredServerFeatures.filter(
      (f) => f.selected
    );

    this.store.dispatch(addServerFeatures({ serverFeatures }));

    this.ref.close();
  };
}
