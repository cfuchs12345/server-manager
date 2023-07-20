import { Component } from '@angular/core';
import { ConfigImExportModalComponent } from './config-im-export-modal/config-im-export-modal.component';
import { MatDialogModule } from '@angular/material/dialog';

@Component({
    selector: 'app-dialog-config-im-export',
    template: '<h1 mat-dialog-title>{{title}}</h1>\
  <div mat-dialog-content>\
    <app-config-im-export-modal></app-config-im-export-modal>\
  </div>',
    standalone: true,
    imports: [MatDialogModule, ConfigImExportModalComponent],
})
export class ConfigImExportDialogComponent {
  title = 'Import and Export of Configuration';
}

export const dialogSettings = () => {
  return {
    height: '800px',
    width: '550px',
  }
}
