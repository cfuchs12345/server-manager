import { Component } from '@angular/core';

@Component({
  selector: 'dialog-config-im-export',
  template: '<h1 mat-dialog-title>{{title}}</h1>\
  <div mat-dialog-content>\
    <app-config-im-export-modal></app-config-im-export-modal>\
  </div>',
})
export class ConfigImExportDialog {
  title = 'Import and Export of Configuration';
}

export const dialogSettings = () => {
  return {
    height: '800px',
    width: '550px',
  }
}
