import { Injectable } from '@angular/core';
import { Param, Plugin, PluginsAction } from './types';
import { Observable, of, take } from 'rxjs';
import { HttpClient } from '@angular/common/http';
import { defaultHeadersForJSON } from '../common';
import { ErrorService, Source } from '../errors/error.service';
import { Store } from '@ngrx/store';
import {
  addOne as addOnePlugin,
  upsertOne as upsertOnePlugin,
  removeOne as removeOnePlugin,
} from 'src/app/state/plugin/plugin.actions';
import {
  addOne as addOneDisabledPlugin,
  upsertOne as upsertOneDisabledPlugin,
  removeOne as removeOneDisabledPlugin,
} from 'src/app/state/disabledplugin/disabled_plugin.actions';
import {
  EventHandler,
  EventHandlingFunction,
  EventHandlingGetObjectFunction,
  EventHandlingUpdateFunction,
} from '../events/types';
import { EventService } from '../events/event.service';
import { EventType } from '../events/types';
import { selectPluginById } from 'src/app/state/plugin/plugin.selectors';

@Injectable({
  providedIn: 'root',
})
export class PluginService {
  // eslint-disable-next-line  @typescript-eslint/no-unused-vars
  insertEventFunction: EventHandlingFunction<Plugin> = (
    eventType: EventType,
    keyType: string,
    key: string,
    value: string,
    object: Plugin
  ) => {
    this.update(key, eventType, object);
  };

  updateEventFunction: EventHandlingUpdateFunction<Plugin> = (
    eventType: EventType,
    keyType: string,
    key: string,
    value: string,
    version: number,
    object: Plugin
  ) => {
    const server$ = this.store.select(selectPluginById(key));
    server$.pipe(take(1)).subscribe({
      next: (plugin) => {
        // only update, if version is different or if the current object in store is preliminary
        if (plugin) {
          this.update(key, eventType, object);
        }
      },
    });
  };

  // eslint-disable-next-line  @typescript-eslint/no-unused-vars
  deleteEventFunction: EventHandlingFunction<Plugin> = (
    eventType: EventType,
    key_name: string,
    key: string,
    value: string // eslint-disable-line @typescript-eslint/no-unused-vars
  ) => {
    this.store.dispatch(removeOnePlugin({ id: key }));
  };

  getObjectFunction: EventHandlingGetObjectFunction<Plugin> = (
    key_name: string,
    key: string,
    value: string, // eslint-disable-line @typescript-eslint/no-unused-vars
    data: string // eslint-disable-line @typescript-eslint/no-unused-vars
  ): Observable<Plugin> => {
    return this.getPlugin(key);
  };

  // eslint-disable-next-line  @typescript-eslint/no-unused-vars
  insertDisabledPluginsEventFunction: EventHandlingFunction<string> = (
    eventType: EventType, // eslint-disable-line @typescript-eslint/no-unused-vars
    keyType: string, // eslint-disable-line @typescript-eslint/no-unused-vars
    key: string, // eslint-disable-line @typescript-eslint/no-unused-vars
    data: string, // eslint-disable-line @typescript-eslint/no-unused-vars
    object: string // eslint-disable-line @typescript-eslint/no-unused-vars
  ) => {
    this.store.dispatch(addOneDisabledPlugin({ disabled_plugin: data }));
  };

  updateDisabledPluginsEventFunction: EventHandlingUpdateFunction<string> = (
    eventType: EventType, // eslint-disable-line @typescript-eslint/no-unused-vars
    keyType: string, // eslint-disable-line @typescript-eslint/no-unused-vars
    key: string, // eslint-disable-line @typescript-eslint/no-unused-vars
    value: string, // eslint-disable-line @typescript-eslint/no-unused-vars
    version: number, // eslint-disable-line @typescript-eslint/no-unused-vars
    object: string // eslint-disable-line @typescript-eslint/no-unused-vars
  ) => {
    this.store.dispatch(upsertOneDisabledPlugin({ disabled_plugin: value }));
  };

  // eslint-disable-next-line  @typescript-eslint/no-unused-vars
  deleteDisabledPluginsEventFunction: EventHandlingFunction<string> = (
    eventType: EventType, // eslint-disable-line @typescript-eslint/no-unused-vars
    key_name: string, // eslint-disable-line @typescript-eslint/no-unused-vars
    key: string, // eslint-disable-line @typescript-eslint/no-unused-vars
    value: string // eslint-disable-line @typescript-eslint/no-unused-vars
  ) => {
    this.store.dispatch(removeOneDisabledPlugin({ disabled_plugin: value }));
  };

  getObjectDisabledPluginsFunction: EventHandlingGetObjectFunction<string> = (
    key_name: string,
    key: string,
    value: string,
    data: string // eslint-disable-line @typescript-eslint/no-unused-vars
  ): Observable<string> => {
    return of(value);
  };

  constructor(
    private store: Store,
    private http: HttpClient,
    private errorService: ErrorService,
    private eventService: EventService
  ) {
    this.eventService.registerEventHandler(
      new EventHandler<Plugin>(
        'Plugin',
        this.insertEventFunction,
        this.updateEventFunction,
        this.deleteEventFunction,
        this.getObjectFunction
      )
    );

    this.eventService.registerEventHandler(
      new EventHandler<string>(
        'DisabledPlugins',
        this.insertDisabledPluginsEventFunction,
        this.updateDisabledPluginsEventFunction,
        this.deleteDisabledPluginsEventFunction,
        this.getObjectDisabledPluginsFunction
      )
    );
  }

  update = (
    userId: string,
    event_type: 'Insert' | 'Update' | 'Refresh' | 'Delete',
    object: Plugin
  ) => {
    if (event_type === 'Insert') {
      this.store.dispatch(addOnePlugin({ plugin: object }));
    } else {
      this.store.dispatch(upsertOnePlugin({ plugin: object }));
    }
  };

  getPlugin = (id: string): Observable<Plugin> => {
    return this.http.get<Plugin>(`/backend/plugins/${id}`);
  };

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
