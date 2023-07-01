import { Component } from "@angular/core";
import { MatDialogRef } from "@angular/material/dialog";

@Component({
  selector: 'dialog-manage-plugins',
  template: '<h1 mat-dialog-title>{{title}}</h1>\
  <div mat-dialog-content>\
    <app-disable-plugins-modal></app-disable-plugins-modal>\
  </div>',
  styles: ['::ng-deep .mat-mdc-dialog-content { max-height: 1000vh;}']
})
export class DisablePluginsDialog {
  title = 'Disable Plugins';
}

export const dialogSettings = () => {
  return {
    width: '550px',
    height: '550px'
  }
}
