import { Injectable } from '@angular/core';
import { Param, Plugin, PluginsAction } from './types';
import { Observable, filter } from 'rxjs';
import { HttpClient } from '@angular/common/http';
import { defaultHeadersForJSON } from '../common';
import { ErrorService, Source } from '../errors/error.service';
import { Store } from '@ngrx/store';
import {
  removeAll as removeAllPlugins,
} from 'src/app/state/plugin/plugin.actions';
import {
  removeAll as removeAllDisabledPlugins,
} from 'src/app/state/disabledplugin/disabled_plugin.actions';
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
    this.eventService.eventSubject$
      .pipe(
        filter((eventAndObject: [Event, Plugin]) => {
          const event = eventAndObject[0];
          return (
            event.object_type === 'Plugin' ||
            event.object_type === 'DisabledPlugins'
          );
        })
      )
      .subscribe((eventAndObject: [Event, Plugin]) => {
        const event = eventAndObject[0];

        if (event.object_type === 'Plugin') {
          this.store.dispatch(removeAllPlugins());
          this.loadPlugins();
        } else if (event.object_type === 'DisabledPlugins') {
          this.store.dispatch(removeAllDisabledPlugins());
          this.loadDisabledPlugins();
        }
      });
  }

  loadPlugins = (): Observable<Plugin[]> => {
    return this.http.get<Plugin[]>('/backend/plugins');
  };

  loadDisabledPlugins = (): Observable<string[]> => {
    return this.http.get<string[]>('/backend/plugins/actions?query=disabled');
  };

  disablePlugins = async (ids: string[]) => {
    const action = new PluginsAction('Disable', [
      new Param('ids', ids.join(',')),
    ]);
    const body = JSON.stringify(action);

    this.http
      .put('/backend/plugins/actions', body, {
        headers: defaultHeadersForJSON(),
      })
      .subscribe({
        error: (err) => {
          this.errorService.newError(Source.PluginService, undefined, err);
        },
      });
  };
}
