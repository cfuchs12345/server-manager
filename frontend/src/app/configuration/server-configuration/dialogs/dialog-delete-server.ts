import { Component } from '@angular/core';
import { DeleteServerModalComponent } from './delete-server-modal/delete-server-modal.component';
import { MatDialogModule } from '@angular/material/dialog';

@Component({
    selector: 'app-dialog-delete-server',
    template: '<h1 mat-dialog-title>{{title}}</h1>\
  <div mat-dialog-content>\
    <app-delete-server-modal></app-delete-server-modal>\
  </div>',
    styles: ['::ng-deep .mat-mdc-dialog-content { max-height: 1000vh;}'],
    standalone: true,
    imports: [MatDialogModule, DeleteServerModalComponent]
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
