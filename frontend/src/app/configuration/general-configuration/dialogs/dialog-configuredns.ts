import { Component } from '@angular/core';
import { MatDialogRef, MAT_DIALOG_DATA } from '@angular/material/dialog';

@Component({
  selector: 'dialog-configure-dns',
  template: '<h1 mat-dialog-title>{{title}}</h1>\
  <div mat-dialog-content>\
    <app-configure-dns-modal></app-configure-dns-modal>\
  </div>',
})
export class ConfigureDNSDialog {
  title: string = 'Configure DNS';
}

export const dialogSettings = () => {
  return {
    height: '800px',
    width: '550px',
  }
}
