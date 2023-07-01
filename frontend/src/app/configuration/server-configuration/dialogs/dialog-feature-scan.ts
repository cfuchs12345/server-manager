import { Component } from '@angular/core';
import { MatDialogRef, MAT_DIALOG_DATA } from '@angular/material/dialog';

@Component({
  selector: 'dialog-feature-scan',
  template: '<h1 mat-dialog-title>{{title}}</h1>\
  <div mat-dialog-content>\
    <app-feature-scan-modal></app-feature-scan-modal>\
  </div>',
   styles: ['::ng-deep .mat-mdc-dialog-content { max-height: 1000vh;}']
})
export class FeatureScanDialog {
  title = 'Feature Scan';
}

export const dialogSettings = () => {
  return {
    width: '500px',
    height: '550px'
  }
}
