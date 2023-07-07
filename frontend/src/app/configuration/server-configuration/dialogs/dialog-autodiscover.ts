import { Component } from '@angular/core';

@Component({
  selector: 'dialog-auto-discovery',
  template: '<h1 mat-dialog-title>{{title}}</h1>\
  <div mat-dialog-content>\
    <app-autodiscover-server-modal></app-autodiscover-server-modal>\
  </div>',
  styles: ['::ng-deep .mat-mdc-dialog-content { max-height: 1000vh;}'],
})
export class AutoDiscoveryDialog {
  title = 'Autodiscovery';
}

export const dialogSettings = () => {
  return {
    width: '550px',
    height: '550px',
  };
};
