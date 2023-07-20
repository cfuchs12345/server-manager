import { Component } from '@angular/core';

@Component({
  selector: 'app-dialog-delete-server',
  template: '<h1 mat-dialog-title>{{title}}</h1>\
  <div mat-dialog-content>\
    <app-delete-server-modal></app-delete-server-modal>\
  </div>',
   styles: ['::ng-deep .mat-mdc-dialog-content { max-height: 1000vh;}']
})
export class DeleteServerDialogComponent {
  title = 'Delete Server / Feature from Server';
}

export const dialogSettings = () => {
  return {
    height: '800px',
    width: '550px',
  }
}
