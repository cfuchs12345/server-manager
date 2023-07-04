import { Component, OnInit, OnDestroy } from '@angular/core';
import { Plugin } from '../../../../services/plugins/types';
import { Subscription } from 'rxjs';
import { Store } from '@ngrx/store';
import { selectAllPlugins } from 'src/app/state/plugin/plugin.selectors';

@Component({
  selector: 'app-list-plugins-modal',
  templateUrl: './list-plugins-modal.component.html',
  styleUrls: ['./list-plugins-modal.component.scss']
})
export class ListPluginsModalComponent implements OnInit, OnDestroy {
  displayedColumns: string[] = ['description', 'detection'];

  plugins: Plugin[] = [];
  subscriptionPlugins: Subscription | undefined = undefined;

  constructor(private store: Store) { }

  ngOnInit() {
    this.subscriptionPlugins = this.store.select(selectAllPlugins).subscribe(plugins => {
      if (plugins) {
        this.plugins = plugins;
      } else {
        // clear messages when empty message received
        this.plugins = [];
      }
    });
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
