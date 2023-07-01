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
  buttonTextScanFeature = 'Start';
  buttonTextWorking = 'Working...';
  buttonTextSaveServerFeatures = 'Save Features';

  displayedColumns = ['selected', 'ipaddress', 'features'];

  isWorking = false;

  discoveredServerFeatures: ServerFeature[] = [];
  subscriptionDiscoveredServersFeatures: Subscription | undefined = undefined;

  constructor(
    private discoveryService: ServerDiscoveryService,
    private serverService: ServerService,
    private ref: MatDialogRef<FeatureScanModalComponent>
  ) {}

  // doesn't seem to work when written as arrow function!?
  ngOnInit(): void {
  }

  ngOnDestroy(): void {
    if (this.subscriptionDiscoveredServersFeatures) {
      this.subscriptionDiscoveredServersFeatures.unsubscribe();
    }
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
    this.subscriptionDiscoveredServersFeatures = this.discoveryService.scanFeatureOfAllServers().subscribe(
      (serverFeatures) => {
        this.isWorking = false;
        this.preSelectAllFeatures(serverFeatures);

        this.discoveredServerFeatures = serverFeatures;
        this.subscriptionDiscoveredServersFeatures?.unsubscribe();
      }
    );
  };

  onClickDeselectServer = (index: number) => {
    this.discoveredServerFeatures[index].selected =
      !this.discoveredServerFeatures[index].selected;
  };

  onClickSaveServerFeatures = () => {
    const found_server_features = this.discoveredServerFeatures.filter((f) => f.selected);

    found_server_features.forEach( (found_server_feature, index) => {

      const subscription = this.serverService.getServer(found_server_feature.ipaddress, true).subscribe({
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
        },
        error: (err) => {

        },
        complete: () => {
          if( subscription ) {
            subscription.unsubscribe();
          }
          if( index === found_server_features.length -1) {
            this.serverService.listServers();
          }
        }
      });
    });


    this.ref.close();
  };
}
