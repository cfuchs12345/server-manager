import { Injectable } from '@angular/core';
import { Param, Plugin, PluginsAction } from './types';
import { HttpClient } from '@angular/common/http';
import { Observable, BehaviorSubject } from 'rxjs';
import { defaultHeadersForJSON } from '../common';
import { ErrorService } from '../errors/error.service';

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
        this._plugins.next(Object.assign({}, this.dataStore).plugins);
      },
      error: (err: any) => {
        this.errorService.newError(this, undefined, err.message);
      },
      complete: () => {},
    });
  };

  loadDisabledPlugins = async () => {
    this.http
      .get<string[]>('/backend/plugins/actions?query=disabled')
      .subscribe({
        next: (idList) => {
          this.dataStore.disabledPlugins = idList;
          this._disabledPlugins.next(
            Object.assign({}, this.dataStore).disabledPlugins
          );
        },
        error: (err: any) => {
          this.errorService.newError(this, undefined,  err.message);
        },
        complete: () => {},
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
        next: (result) => {
          this.loadDisabledPlugins();
        },
        error: (err: any) => {
          this.errorService.newError(this,  undefined, err.message);
        },
        complete: () => {},
      });
  };
}
