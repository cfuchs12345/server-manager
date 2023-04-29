import { Component, OnDestroy, OnInit } from "@angular/core";
import { MatDialogRef } from "@angular/material/dialog";
import { Subscription } from "rxjs";
import { PluginService } from "src/app/services/plugins/plugin.service";
import { Plugin } from "src/app/services/plugins/types";


@Component({
  selector: 'dialog-list-plugins',
  template: '<h1 mat-dialog-title>{{  title }}</h1>\
  <div mat-dialog-content>\
    <app-list-plugins-modal></app-list-plugins-modal>\
  </div>',
  styles: ['::ng-deep .mat-mdc-dialog-content { max-height: 1000vh;}']
})
export class ListPluginsDialog {
  title: string = 'List Plugins';
}

export const dialogSettings = () => {
  return {
    height: '800px',
     width: '550px',
  }
}
