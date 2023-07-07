import { Component, OnInit, OnDestroy } from '@angular/core';
import { Store } from '@ngrx/store';
import { Observable, map } from 'rxjs';
import { PluginService } from 'src/app/services/plugins/plugin.service';
import { Plugin } from 'src/app/services/plugins/types';
import { SubscriptionHandler } from 'src/app/shared/subscriptionHandler';
import { selectAllDisabledPlugins } from 'src/app/state/disabledplugin/disabled_plugin.selectors';
import { selectAllPlugins } from 'src/app/state/plugin/plugin.selectors';

@Component({
  selector: 'app-disable-plugins-modal',
  templateUrl: './disable-plugins-modal.component.html',
  styleUrls: ['./disable-plugins-modal.component.scss'],
})
export class DisablePluginsModalComponent implements OnInit, OnDestroy {
  buttonTextDisablePlugins = 'Disable Plugins';
  selectAll = false;
  displayedColumns: string[] = ['disable', 'name'];
  plugins$: Observable<Plugin[]>;

  private disabledPlugins$: Observable<string[]>;

  private selectedPlugins: string[] = [];

  private subscriptionHandler = new SubscriptionHandler(this);

  constructor(private store: Store, private servicePlugins: PluginService) {
    this.plugins$ = this.store.select(selectAllPlugins);
    this.disabledPlugins$ = this.store.select(selectAllDisabledPlugins);
  }

  ngOnInit(): void {
    this.subscriptionHandler.subscription = this.disabledPlugins$.subscribe(
      (disablePlugins) => (this.selectedPlugins = disablePlugins)
    );
  }

  ngOnDestroy(): void {
    this.subscriptionHandler.onDestroy();
  }

  isDisabled = (id: string): boolean => {
    return this.selectedPlugins.indexOf(id) > -1;
  };

  onClickSaveDisabledPlugins = () => {
    this.servicePlugins.disablePlugins(this.selectedPlugins);
  };

  onClickSelectPlugin = (plugin: Plugin) => {
    if (this.isDisabled(plugin.id)) {
      this.selectedPlugins = this.selectedPlugins.filter(
        (str) => str !== plugin.id
      );
    } else {
      this.selectedPlugins.push(plugin.id);
    }
  };

  onClickSelectAll = () => {
    this.selectAll = !this.selectAll;

    if (this.selectAll) {
      this.subscriptionHandler.subscription = this.plugins$
        .pipe(map((plugins) => plugins.map((p) => p.id)))
        .subscribe((pluginames) => (this.selectedPlugins = pluginames));
    } else {
      this.selectedPlugins = [];
    }
  };
}
