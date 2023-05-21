import { Component } from '@angular/core';
import { MatDialogRef, MAT_DIALOG_DATA } from '@angular/material/dialog';

@Component({
  selector: 'dialog-configure-users',
  template: '<h1 mat-dialog-title>{{title}}</h1>\
  <div mat-dialog-content>\
    <app-configure-users-modal></app-configure-users-modal>\
  </div>',
})
export class ConfigureUsersDialog {
  title: string = 'Configure Users';
}

export const dialogSettings = () => {
  return {
    height: '800px',
    width: '550px',
  }
}
