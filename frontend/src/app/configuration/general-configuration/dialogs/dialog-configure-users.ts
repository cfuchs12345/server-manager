import { Component } from '@angular/core';
import { ConfigureUsersModalComponent } from './configure-users-modal/configure-users-modal.component';
import { MatDialogModule } from '@angular/material/dialog';

@Component({
    selector: 'app-dialog-configure-users',
    template: '<h1 mat-dialog-title>{{title}}</h1>\
  <div mat-dialog-content>\
    <app-configure-users-modal></app-configure-users-modal>\
  </div>',
    standalone: true,
    imports: [MatDialogModule, ConfigureUsersModalComponent],
})
export class ConfigureUsersDialogComponent {
  title = 'Configure Users';
}

export const dialogSettings = () => {
  return {
    height: '800px',
    width: '550px',
  }
}
