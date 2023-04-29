import { Component } from '@angular/core';
import { MatDialogRef, MAT_DIALOG_DATA } from '@angular/material/dialog';

@Component({
  selector: 'dialog-add-server',
  template: '<h1 mat-dialog-title>{{title}}</h1>\
  <div mat-dialog-content>\
    <app-add-server-modal></app-add-server-modal>\
  </div>',
   styles: ['::ng-deep .mat-mdc-dialog-content { max-height: 1000vh;}']
})
export class AddServerDialog {
  title: string = 'Add Server / Feature to Server';
}

export const dialogSettings = () => {
  return {
    height: '800px',
    width: '550px',
  }
}
