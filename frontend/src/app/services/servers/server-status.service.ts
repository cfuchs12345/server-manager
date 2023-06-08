import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { BehaviorSubject } from 'rxjs';
import { defaultHeadersForJSON } from '../common';
import {
  Server,
  ServersAction,
} from './types';

import { Status } from './types';
import { ErrorService } from '../errors/error.service';

@Injectable({
  providedIn: 'root',
})
export class ServerStatusService {
  private _serverStatus = new BehaviorSubject<Status[]>([]);
  readonly serversStatus = this._serverStatus.asObservable();

  private dataStore: {
    serversStatus: Status[];
  } = {
    serversStatus: [],
  };

  constructor(private http: HttpClient, private errorService: ErrorService) {}

  listServerStatus = (servers: Server[]) => {
    const action = new ServersAction('Status', []);
    const body = JSON.stringify(action);

    this.http
      .post<Status[]>('/backend/servers/actions', body, {
        headers: defaultHeadersForJSON(),
      })
      .subscribe({
        next: (statusList) => {
          this.dataStore.serversStatus.splice(
            0,
            this.dataStore.serversStatus.length
          );
          this.dataStore.serversStatus.push(...statusList);
          this.publishServerStatus();
        },
        error: (err: any) => {
          if (err !== undefined) {
            this.errorService.newError(
              'Status-Service',
              undefined,
              err.message
            );
          }
        },
        complete: () => {},
      });
  };

  private publishServerStatus = () => {
    this._serverStatus.next(Object.assign({}, this.dataStore).serversStatus);
  };
}
