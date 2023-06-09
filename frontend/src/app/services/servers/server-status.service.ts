import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { BehaviorSubject } from 'rxjs';
import { defaultHeadersForJSON } from '../common';
import { Server, ServersAction } from './types';

import { Status } from './types';
import { ErrorService, Source } from '../errors/error.service';

@Injectable({
  providedIn: 'root',
})
export class ServerStatusService {
  private _serverStatus = new BehaviorSubject<Status[]>([]);
  readonly serversStatus = this._serverStatus.asObservable();

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
          this.publishServerStatus(statusList);
        },
        error: (err: any) => {
          if (err) {
            this.errorService.newError(
              Source.ServerStatusService,
              undefined,
              err
            );
          }
        },
        complete: () => {},
      });
  };

  private publishServerStatus = (list: Status[]) => {
    this._serverStatus.next(list);
  };
}
