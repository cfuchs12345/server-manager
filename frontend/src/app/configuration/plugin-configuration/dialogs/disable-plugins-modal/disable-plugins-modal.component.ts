import { Component, OnInit, OnDestroy } from '@angular/core';
import { MatDialogRef } from '@angular/material/dialog';
import { Subscription } from 'rxjs';
import { PluginService } from 'src/app/services/plugins/plugin.service';
import { Plugin } from 'src/app/services/plugins/types';

@Component({
  selector: 'app-disable-plugins-modal',
  templateUrl: './disable-plugins-modal.component.html',
  styleUrls: ['./disable-plugins-modal.component.scss'],
})
export class DisablePluginsModalComponent implements OnInit, OnDestroy {
  buttonTextDisablePlugins: string = 'Disable Plugins';

  selectAll: boolean = false;

  displayedColumns: string[] = ['disable', 'name'];

  plugins: Plugin[] = [];
  disabledPlugins: string[] = [];

  subscriptionPlugins: Subscription | undefined = undefined;
  subscriptionDisabledPlugins: Subscription | undefined = undefined;

  constructor(private servicePlugins: PluginService) {}

  ngOnInit() {
    this.subscriptionPlugins = this.servicePlugins.plugins.subscribe(
      (plugins) => {
        if (plugins) {
          this.plugins = plugins;
        } else {
          // clear messages when empty message received
          this.plugins = [];
        }
      }
    );

    this.subscriptionDisabledPlugins = this.servicePlugins
      .loadDisabledPlugins()
      .subscribe({
        next: (disabledPlugins) => {
          if (disabledPlugins) {
            this.disabledPlugins = disabledPlugins;
          } else {
            this.disabledPlugins = [];
          }
        },
        complete: () => {
          setTimeout(() => {
            this.subscriptionDisabledPlugins?.unsubscribe();
          }, 50);
        },
      });

    this.servicePlugins.loadPlugins();
  }

  ngOnDestroy(): void {
    if (this.subscriptionPlugins) {
      this.subscriptionPlugins.unsubscribe();
    }
    if (this.subscriptionDisabledPlugins) {
      this.subscriptionDisabledPlugins.unsubscribe();
    }
  }

  isDisabled = (id: string): boolean => {
    return this.disabledPlugins.indexOf(id) >= 0;
  };

  disablePlugins = () => {
    this.servicePlugins.disablePlugins(this.disabledPlugins);
  };

  onClickSelectPlugin = (plugin: Plugin) => {
    if (this.isDisabled(plugin.id)) {
      this.disabledPlugins = this.disabledPlugins.filter(
        (str) => str !== plugin.id
      );
    } else {
      this.disabledPlugins.push(plugin.id);
    }
  };

  onClickSelectAll = () => {
    const list: string[] = [];
    this.selectAll = !this.selectAll;

    if (this.selectAll) {
      for (const plugin of this.plugins) {
        list.push(plugin.id);
      }
    }
    this.disabledPlugins = list;
  };
}
