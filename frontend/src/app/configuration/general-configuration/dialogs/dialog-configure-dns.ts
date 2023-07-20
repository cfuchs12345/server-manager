import { Component } from '@angular/core';
import { ConfigureDnsModalComponent } from './configure-dns-modal/configure-dns-modal.component';
import { MatDialogModule } from '@angular/material/dialog';

@Component({
    selector: 'app-dialog-configure-dns',
    template: '<h1 mat-dialog-title>{{title}}</h1>\
  <div mat-dialog-content>\
    <app-configure-dns-modal></app-configure-dns-modal>\
  </div>',
    standalone: true,
    imports: [MatDialogModule, ConfigureDnsModalComponent],
})
export class ConfigureDNSDialogComponent {
  title = 'Configure DNS';
}

export const dialogSettings = () => {
  return {
    height: '800px',
    width: '550px',
  }
}
