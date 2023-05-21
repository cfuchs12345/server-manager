import { Component } from '@angular/core';
import { MatDialogRef, MAT_DIALOG_DATA } from '@angular/material/dialog';

@Component({
  selector: 'dialog-change-password',
  template: '<h1 mat-dialog-title>{{title}}</h1>\
  <div mat-dialog-content>\
    <app-change-password-modal></app-change-password-modal>\
  </div>',
})
export class ChangePasswordDialog {
  title: string = 'Change Password';
}

export const dialogSettings = () => {
  return {
    height: '800px',
    width: '550px',
  }
}
