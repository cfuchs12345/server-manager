import { Component, OnInit, OnDestroy } from '@angular/core';
import { Plugin } from '../../../../services/plugins/types';
import { PluginService } from 'src/app/services/plugins/plugin.service';
import { MatDialogRef } from '@angular/material/dialog';
import { Subscription } from 'rxjs';

@Component({
  selector: 'app-list-plugins-modal',
  templateUrl: './list-plugins-modal.component.html',
  styleUrls: ['./list-plugins-modal.component.scss']
})
export class ListPluginsModalComponent implements OnInit, OnDestroy {
  displayedColumns: string[] = ['description', 'detection'];

  plugins: Plugin[] = [];
  subscriptionPlugins: Subscription | undefined = undefined;

  constructor(private servicePlugins: PluginService) { }

  ngOnInit() {
    this.subscriptionPlugins = this.servicePlugins.plugins.subscribe(plugins => {
      if (plugins) {
        this.plugins = plugins;
      } else {
        // clear messages when empty message received
        this.plugins = [];
      }
    });

    this.servicePlugins.loadPlugins();
  }

  ngOnDestroy(): void {
    if( this.subscriptionPlugins ) {
      this.subscriptionPlugins.unsubscribe();
    }
  }

  detectionPossible(plugin: Plugin): boolean {
    return plugin && plugin.detection && plugin.detection.detection_possible && plugin.detection.detection_possible === true;
  }
}
