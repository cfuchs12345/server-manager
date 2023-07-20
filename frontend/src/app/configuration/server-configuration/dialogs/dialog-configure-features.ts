import { Component } from '@angular/core';
import { ConfigureFeaturesModalComponent } from './configure-features-modal/configure-features-modal.component';
import { MatDialogModule } from '@angular/material/dialog';

@Component({
    selector: 'app-dialog-configure-features',
    template: '<h1 mat-dialog-title>{{title}}</h1>\
  <div mat-dialog-content>\
    <app-configure-features-modal></app-configure-features-modal>\
  </div>',
    standalone: true,
    imports: [MatDialogModule, ConfigureFeaturesModalComponent],
})
export class ConfigureFeaturesDialogComponent {
  title = 'Configure Features';
}

export const dialogSettings = () => {
  return {
    height: '800px',
    width: '550px',
  }
}
