import { Component } from '@angular/core';
import { MatDialogRef, MAT_DIALOG_DATA } from '@angular/material/dialog';

@Component({
  selector: 'dialog-configure-features',
  template: '<h1 mat-dialog-title>{{title}}</h1>\
  <div mat-dialog-content>\
    <app-configure-features-modal></app-configure-features-modal>\
  </div>',
})
export class ConfigureFeaturesDialog {
  title = 'Configure Features';
}

export const dialogSettings = () => {
  return {
    height: '800px',
    width: '550px',
  }
}
