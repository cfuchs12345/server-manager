import { Component } from '@angular/core';
import { FeatureScanModalComponent } from './feature-scan-modal/feature-scan-modal.component';
import { MatDialogModule } from '@angular/material/dialog';

@Component({
    selector: 'app-dialog-feature-scan',
    template: '<h1 mat-dialog-title>{{title}}</h1>\
  <div mat-dialog-content>\
    <app-feature-scan-modal></app-feature-scan-modal>\
  </div>',
    styles: ['::ng-deep .mat-mdc-dialog-content { max-height: 1000vh;}'],
    standalone: true,
    imports: [MatDialogModule, FeatureScanModalComponent]
})
export class FeatureScanDialogComponent {
  title = 'Feature Scan';
}

export const dialogSettings = () => {
  return {
    width: '500px',
    height: '550px'
  }
}
