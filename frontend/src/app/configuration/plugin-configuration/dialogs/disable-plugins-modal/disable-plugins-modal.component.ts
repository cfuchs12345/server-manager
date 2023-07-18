import {
  Component,
  OnInit,
  OnDestroy,
  OnChanges,
  SimpleChanges,
  inject,
  ChangeDetectorRef,
} from '@angular/core';
import { Store } from '@ngrx/store';
import { Observable, map, of, take } from 'rxjs';
import { Plugin } from 'src/app/services/plugins/types';
import { SubscriptionHandler } from 'src/app/shared/subscriptionHandler';
import {
  addMany,
  addOne,
  disablePlugins,
  removeAll,
  removeOne,
} from 'src/app/state/disabledplugin/disabled_plugin.actions';
import { selectAllDisabledPlugins } from 'src/app/state/disabledplugin/disabled_plugin.selectors';
import { selectAllPlugins } from 'src/app/state/plugin/plugin.selectors';

@Component({
  selector: 'app-disable-plugins-modal',
  templateUrl: './disable-plugins-modal.component.html',
  styleUrls: ['./disable-plugins-modal.component.scss'],
})
export class DisablePluginsModalComponent
  implements OnInit, OnDestroy, OnChanges
{
  private store = inject(Store);
  private cdr = inject(ChangeDetectorRef);
  private subscriptionHandler = new SubscriptionHandler(this);

  buttonTextDisablePlugins = 'Disable Plugins';
  selectAll = false;
  displayedColumns: string[] = ['disable', 'name'];
  plugins$?: Observable<Plugin[]>;

  private disabledPlugins$: Observable<string[]> = of([]);

  ngOnInit(): void {
    this.plugins$ = this.store.select(selectAllPlugins);
    this.disabledPlugins$ = this.store.select(selectAllDisabledPlugins);
  }

  ngOnChanges(simpleChange: SimpleChanges): void {
    console.log('simpleChange', simpleChange);
  }

  ngOnDestroy(): void {
    this.subscriptionHandler.onDestroy();
  }

  onClickSaveDisabledPlugins = () => {
    this.subscriptionHandler.subscription = this.disabledPlugins$
      .pipe(take(1))
      .subscribe((plugins) => {
        this.store.dispatch(disablePlugins({ plugins }));
      });
  };

  isDisabled = (plugin: Plugin): Observable<boolean> => {
    return this.disabledPlugins$.pipe(
      take(1),
      map((pluginIds) => pluginIds.indexOf(plugin.id) > -1)
    );
  };

  onClickSelectPlugin = (plugin: Plugin) => {
    return this.disabledPlugins$
      .pipe(
        take(1),
        map((pluginIds) => pluginIds.indexOf(plugin.id) > -1)
      )
      .subscribe((exists) => {
        if (exists) {
          this.store.dispatch(removeOne({ disabled_plugin: plugin.id }));
        } else {
          this.store.dispatch(addOne({ disabled_plugin: plugin.id }));
        }
      });
  };

  onClickSelectAll = () => {
    this.selectAll = !this.selectAll;

    this.store.dispatch(removeAll());

    if (this.selectAll && this.plugins$) {
      this.subscriptionHandler.subscription = this.plugins$
        .pipe(
          take(1),
          map((plugins) => plugins.map((p) => p.id))
        )
        .subscribe((pluginIds) => {
          this.store.dispatch(addMany({ disabled_plugins: pluginIds }));
        });
    }
  };
}
