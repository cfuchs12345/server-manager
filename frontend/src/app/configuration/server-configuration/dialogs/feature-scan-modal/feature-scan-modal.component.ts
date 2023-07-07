import { Component, OnDestroy } from '@angular/core';
import { ServerService } from 'src/app/services/servers/server.service';
import { ServerFeature } from 'src/app/services/servers/types';
import { MatDialogRef } from '@angular/material/dialog';
import { ServerDiscoveryService } from 'src/app/services/servers/server-discovery.service';
import { SubscriptionHandler } from 'src/app/shared/subscriptionHandler';

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
    private serverService: ServerService,
    private ref: MatDialogRef<FeatureScanModalComponent>
  ) {}


  ngOnDestroy(): void {
   this.subscriptionHandler.onDestroy();
  }

  private preSelectAllFeatures = (serverFeatures: ServerFeature[]) => {
    serverFeatures.forEach((f) => f.selected = true);
  }


  featuresFound = (): boolean => {
    return this.discoveredServerFeatures.length > 0;
  };

  getFeaturesAsString = (feature: ServerFeature) => {
    return feature.features.map((feature) => feature.name).join(', ');
  };

  onClickScanFeature = () => {
    this.isWorking = true;
    this.subscriptionHandler.subscription = this.discoveryService.scanFeatureOfAllServers().subscribe(
      (serverFeatures) => {
        this.isWorking = false;
        this.preSelectAllFeatures(serverFeatures);

        this.discoveredServerFeatures = serverFeatures;
      }
    );
  };

  onClickDeselectServer = (index: number) => {
    this.discoveredServerFeatures[index].selected =
      !this.discoveredServerFeatures[index].selected;
  };

  onClickSaveServerFeatures = () => {
    const found_server_features = this.discoveredServerFeatures.filter((f) => f.selected);

    found_server_features.forEach( (found_server_feature, ) => {

      this.subscriptionHandler.subscription = this.serverService.getServer(found_server_feature.ipaddress, true).subscribe({
        next: (server) => {
          let updated = false;

          found_server_feature.features.forEach( (found) => {
            const already_set = server.features.find((server_feature) => server_feature.id === found.id)

            if( !already_set) {
              updated = true;
              server.features.push(found);
            }
          }); // we just add new features found, removal is only done manually

          if( updated ) {
            this.serverService.updateServer(server);
          }
        }
      });
    });
    this.ref.close();
  };
}
