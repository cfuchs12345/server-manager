import { Injectable } from '@angular/core';
import { Param, Plugin, PluginsAction } from './types';
import { HttpClient } from '@angular/common/http';
import { BehaviorSubject } from 'rxjs';
import { defaultHeadersForJSON } from '../common';
import { ErrorService, Source } from '../errors/error.service';

@Injectable({
  providedIn: 'root',
})
export class PluginService {
  private _plugins = new BehaviorSubject<Plugin[]>([]);
  private _disabledPlugins = new BehaviorSubject<string[]>([]);
  private dataStore: { plugins: Plugin[]; disabledPlugins: string[] } = {
    plugins: [],
    disabledPlugins: [],
  };

  readonly plugins = this._plugins.asObservable();
  readonly disabledPlugins = this._disabledPlugins.asObservable();

  constructor(private http: HttpClient, private errorService: ErrorService) {}

  loadPlugins = async () => {
    this.http.get<Plugin[]>('/backend/plugins').subscribe({
      next: (loadedPlugins) => {
        this.dataStore.plugins = loadedPlugins;
      },
      error: (err: any) => {
        this.errorService.newError(Source.PluginService, undefined, err);
      },
      complete: () => {
        this.publishPlugins();
      },
    });
  };

  loadDisabledPlugins = async () => {
    this.http
      .get<string[]>('/backend/plugins/actions?query=disabled')
      .subscribe({
        next: (idList) => {
          this.dataStore.disabledPlugins = idList;
        },
        error: (err: any) => {
          this.errorService.newError(Source.PluginService, undefined, err);
        },
        complete: () => {
          this.publishDisabledPlugins();
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
        complete: () => {
          this.loadDisabledPlugins();
        },
      });
  };

  private publishDisabledPlugins = () => {
    this._disabledPlugins.next(this.dataStore.disabledPlugins.slice());
  };

  private publishPlugins = () => {
    this._plugins.next(this.dataStore.plugins.slice());
  };

  getPlugin = (id: string): Plugin | undefined => {
    return this.dataStore.plugins.find((p) => p.id === id);
  };
}
