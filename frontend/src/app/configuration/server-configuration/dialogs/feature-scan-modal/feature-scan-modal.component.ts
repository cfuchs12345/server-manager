import { Component, OnDestroy } from '@angular/core';
import { ServerFeature } from 'src/app/services/servers/types';
import { MatDialogRef } from '@angular/material/dialog';
import { ServerDiscoveryService } from 'src/app/services/servers/server-discovery.service';
import { SubscriptionHandler } from 'src/app/shared/subscriptionHandler';
import { Store } from '@ngrx/store';
import { addServerFeatures } from 'src/app/state/server/server.actions';

@Component({
  selector: 'app-feature-scan-modal',
  templateUrl: './feature-scan-modal.component.html',
  styleUrls: ['./feature-scan-modal.component.scss'],
})
export class FeatureScanModalComponent implements OnDestroy {
  buttonTextScanFeature = 'Start';
  buttonTextWorking = 'Working...';
  buttonTextSaveServerFeatures = 'Save Features';

  displayedColumns = ['selected', 'ipaddress', 'features'];

  isWorking = false;

  discoveredServerFeatures: ServerFeature[] = [];
  subscriptionHandler = new SubscriptionHandler(this);

  constructor(
    private discoveryService: ServerDiscoveryService,
    private store: Store,
    private ref: MatDialogRef<FeatureScanModalComponent>
  ) {}

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
