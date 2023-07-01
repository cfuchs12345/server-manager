import { Injectable } from '@angular/core';
import { Param, Plugin, PluginsAction } from './types';
import { filter } from 'rxjs';
import { HttpClient } from '@angular/common/http';
import { defaultHeadersForJSON } from '../common';
import { ErrorService, Source } from '../errors/error.service';
import { Store } from '@ngrx/store';
import { addMany as addManyPlugins, removeAll as removeAllPlugins } from 'src/app/state/actions/plugin.action';
import { addMany as addManyDisabledPlugins, removeAll as removeAllDisabledPlugins } from 'src/app/state/actions/disabled_plugin.action';
import { EventService } from '../events/event.service';
import { Event } from '../events/types';

@Injectable({
  providedIn: 'root',
})
export class PluginService {
  constructor(
    private store: Store,
    private http: HttpClient,
    private errorService: ErrorService,
    private eventService: EventService
  ) {
    this.eventService.eventSubject
      .pipe(
        filter((event: Event) => {
          return (
            event.object_type === 'Plugin' ||
            event.object_type === 'DisabledPlugins'
          );
        })
      )
      .subscribe((event: Event) => {
        if (event.object_type === 'Plugin') {
            this.store.dispatch( removeAllPlugins() );
            this.loadPlugins();
        }
        else if (event.object_type === 'DisabledPlugins') {
          this.store.dispatch( removeAllDisabledPlugins() );
          this.loadDisabledPlugins();
        }
      });
  }

  loadPlugins = async () => {
    const subscription = this.http.get<Plugin[]>('/backend/plugins').subscribe({
      next: (loadedPlugins) => {
        this.store.dispatch(addManyPlugins({ plugins: loadedPlugins }));
      },
      error: (err: any) => {
        this.errorService.newError(Source.PluginService, undefined, err);
      },
      complete: () => {
        subscription.unsubscribe();
      },
    });
  };

  loadDisabledPlugins = async () => {
    const subscription = this.http
      .get<string[]>('/backend/plugins/actions?query=disabled')
      .subscribe({
        next: (disabledPlugins) => {
          this.store.dispatch(
            addManyDisabledPlugins({ disabled_plugins: disabledPlugins })
          );
        },
        error: (err: any) => {
          this.errorService.newError(Source.PluginService, undefined, err);
        },
        complete: () => {
          subscription.unsubscribe();
        },
      });
  };

  disablePlugins = async (ids: string[]) => {
    const action = new PluginsAction('Disable', [
      new Param('ids', ids.join(',')),
    ]);
    const body = JSON.stringify(action);

    this.http
      .put<any>('/backend/plugins/actions', body, {
        headers: defaultHeadersForJSON(),
      })
      .subscribe({
        next: (result) => {},
        error: (err: any) => {
          this.errorService.newError(Source.PluginService, undefined, err);
        },
        complete: () => {},
      });
  };
}
