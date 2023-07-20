import { Component } from '@angular/core';
import { AddServerModalComponent } from './add-server-modal/add-server-modal.component';
import { MatDialogModule } from '@angular/material/dialog';

@Component({
    selector: 'app-dialog-add-server',
    template: '<h1 mat-dialog-title>{{title}}</h1>\
  <div mat-dialog-content>\
    <app-add-server-modal></app-add-server-modal>\
  </div>',
    styles: ['::ng-deep .mat-mdc-dialog-content { max-height: 1000vh;}'],
    standalone: true,
    imports: [MatDialogModule, AddServerModalComponent]
})
export class AddServerDialogComponent {
  title = 'Add Server / Feature to Server';
}

export const dialogSettings = () => {
  return {
    height: '800px',
    width: '550px',
  }
}
