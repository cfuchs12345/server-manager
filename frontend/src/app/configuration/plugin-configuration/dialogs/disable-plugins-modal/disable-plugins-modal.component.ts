import { Component, OnInit, OnDestroy } from '@angular/core';
import { MatDialogRef } from '@angular/material/dialog';
import { Store } from '@ngrx/store';
import { Subscription } from 'rxjs';
import { PluginService } from 'src/app/services/plugins/plugin.service';
import { Plugin } from 'src/app/services/plugins/types';
import { selectAllDisabledPlugins } from 'src/app/state/selectors/disabled_plugin.selectors';
import { selectAllPlugins } from 'src/app/state/selectors/plugin.selectors';

@Component({
  selector: 'app-disable-plugins-modal',
  templateUrl: './disable-plugins-modal.component.html',
  styleUrls: ['./disable-plugins-modal.component.scss'],
})
export class DisablePluginsModalComponent implements OnInit, OnDestroy {
  buttonTextDisablePlugins = "Disable Plugins";

  selectAll = false;

  displayedColumns: string[] = ['disable', 'name'];

  plugins: Plugin[] = [];
  disabledPlugins: string[] = [];

  subscriptionPlugins: Subscription | undefined = undefined;
  subscriptionDisabledPlugins: Subscription | undefined = undefined;

  constructor(private store: Store, private servicePlugins: PluginService) {}

  ngOnInit() {
    this.subscriptionPlugins = this.store.select(selectAllPlugins).subscribe(
      (plugins) => {
        if (plugins) {
          this.plugins = plugins;
        } else {
          // clear messages when empty message received
          this.plugins = [];
        }
      }
    );

    this.subscriptionDisabledPlugins =
      this.store.select(selectAllDisabledPlugins).subscribe((disabledPlugins) => {
        if (disabledPlugins) {
          this.disabledPlugins = disabledPlugins;
        } else {
          this.disabledPlugins = [];
        }
      });
  }

  ngOnDestroy(): void {
    if (this.subscriptionPlugins) {
      this.subscriptionPlugins.unsubscribe();
    }
    if (this.subscriptionDisabledPlugins) {
      this.subscriptionDisabledPlugins.unsubscribe();
    }
  }


  isDisabled = (id: string):boolean => {
    return this.disabledPlugins.indexOf(id) >= 0;
  }

  disablePlugins = () => {
    this.servicePlugins.disablePlugins(this.disabledPlugins);
  }

  onClickSelectPlugin = (plugin: Plugin) => {
    if( this.isDisabled(plugin.id)) {
      this.disabledPlugins = this.disabledPlugins.filter( (str) => str !== plugin.id);
    }
    else {
      this.disabledPlugins.push(plugin.id);
    }
  }

  onClickSelectAll = () => {
    const list:string[] = [];
    this.selectAll = !this.selectAll;

    if( this.selectAll ) {
      for( const plugin of this.plugins) {
        list.push(plugin.id);
      }
    }
    this.disabledPlugins = list;
  }
}
