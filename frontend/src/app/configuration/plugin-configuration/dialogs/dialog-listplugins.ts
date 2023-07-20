import { Component } from "@angular/core";


@Component({
  selector: 'app-dialog-list-plugins',
  template: '<h1 mat-dialog-title>{{  title }}</h1>\
  <div mat-dialog-content>\
    <app-list-plugins-modal></app-list-plugins-modal>\
  </div>',
  styles: ['::ng-deep .mat-mdc-dialog-content { max-height: 1000vh;}']
})
export class ListPluginsDialogComponent {
  title = 'List Plugins';
}

export const dialogSettings = () => {
  return {
    height: '800px',
     width: '550px',
  }
}
