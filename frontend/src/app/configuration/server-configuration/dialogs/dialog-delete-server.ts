import { Component } from '@angular/core';
import { MatDialogRef, MAT_DIALOG_DATA } from '@angular/material/dialog';

@Component({
  selector: 'dialog-delete-server',
  template: '<h1 mat-dialog-title>{{title}}</h1>\
  <div mat-dialog-content>\
    <app-delete-server-modal></app-delete-server-modal>\
  </div>',
   styles: ['::ng-deep .mat-mdc-dialog-content { max-height: 1000vh;}']
})
export class DeleteServerDialog {
  title: string = 'Delete Server / Feature from Server';
}

export const dialogSettings = () => {
  return {
    height: '800px',
    width: '550px',
  }
}
