import { Component } from '@angular/core';
import { ChangePasswordModalComponent } from './change-password-modal/change-password-modal.component';
import { MatDialogModule } from '@angular/material/dialog';

@Component({
    selector: 'app-dialog-change-password',
    template: '<h1 mat-dialog-title>{{title}}</h1>\
  <div mat-dialog-content>\
    <app-change-password-modal></app-change-password-modal>\
  </div>',
    standalone: true,
    imports: [MatDialogModule, ChangePasswordModalComponent],
})
export class ChangePasswordDialogComponent {
  title = 'Change Password';
}

export const dialogSettings = () => {
  return {
    height: '800px',
    width: '550px',
  }
}
