import { Component, OnDestroy, OnInit } from '@angular/core';
import { ServerService } from 'src/app/services/servers/server.service';
import { ServerFeature } from 'src/app/services/servers/types';
import { Subscription } from 'rxjs';
import { MatDialogRef } from '@angular/material/dialog';
import { ServerDiscoveryService } from 'src/app/services/servers/server-discovery.service';

@Component({
  selector: 'app-feature-scan-modal',
  templateUrl: './feature-scan-modal.component.html',
  styleUrls: ['./feature-scan-modal.component.scss'],
})
export class FeatureScanModalComponent implements OnInit, OnDestroy {
  buttonTextScanFeature: string = 'Start';
  buttonTextWorking: string = 'Working...';
  buttonTextSaveServerFeatures: string = 'Save Features';

  displayedColumns = ['selected', 'ipaddress', 'features'];

  isWorking: boolean = false;

  discoveredServerFeatures: ServerFeature[] = [];
  subscriptionDiscoveredServersFeatures: Subscription | undefined = undefined;

  constructor(
    private discoveryService: ServerDiscoveryService,
    private serverService: ServerService,
    private ref: MatDialogRef<FeatureScanModalComponent>
  ) {}

  // doesn't seem to work when written as arrow function!?
  ngOnInit(): void {
    this.subscriptionDiscoveredServersFeatures = this.discoveryService.discoveredServerFeatures.subscribe(
      (serverFeatures) => {
        this.isWorking = false;
        this.preSelectAllFeatures(serverFeatures);

        this.discoveredServerFeatures = serverFeatures;
      }
    );
  }

  ngOnDestroy(): void {
    if (this.subscriptionDiscoveredServersFeatures) {
      this.subscriptionDiscoveredServersFeatures.unsubscribe();
    }
    this.discoveryService.resetDiscoveredServerFeatures();
  }

  private preSelectAllFeatures = (serverFeatures: ServerFeature[]) => {
    for (let i = 0; i < serverFeatures.length; i++) {
      serverFeatures[i].selected = true;
    }
  }


  featuresFound = (): boolean => {
    return this.discoveredServerFeatures.length > 0;
  };

  getFeaturesAsString = (feature: ServerFeature) => {
    return feature.features.map((feature) => feature.name).join(', ');
  };

  onClickScanFeature = () => {
    this.isWorking = true;
    this.discoveryService.scanFeatureOfAllServers();
  };

  onClickDeselectServer = (index: number) => {
    this.discoveredServerFeatures[index].selected =
      !this.discoveredServerFeatures[index].selected;
  };

  onClickSaveServerFeatures = () => {
    const soSave = this.discoveredServerFeatures.filter((f) => f.selected);

    this.serverService.updateServerFeatures(soSave, false);

    this.ref.close();
  };
}
