import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { defaultHeadersForJSON } from '../common';
import { ServersAction } from './types';

import { Status } from './types';
import { ErrorService, Source } from '../errors/error.service';
import {
  EventHandler,
  EventHandlingFunction,
  EventHandlingUpdateFunction,
  EventService,
} from '../events/event.service';
import { EventType } from '../events/types';
import { NGXLogger } from 'ngx-logger';
import {
  addMany,
  removeOne,
  upsertOne,
} from 'src/app/state/status/status.actions';
import { Store } from '@ngrx/store';

@Injectable({
  providedIn: 'root',
})
export class ServerStatusService {
  // eslint-disable-next-line  @typescript-eslint/no-unused-vars
  insertEventFunction: EventHandlingFunction = (
    eventType: EventType,
    keyType: string,
    key: string,
    data: string
  ) => {
    const newStatus: Status = JSON.parse(data);

    if (newStatus) {
      this.store.dispatch(upsertOne({ status: newStatus }));
    }
  };

  // eslint-disable-next-line  @typescript-eslint/no-unused-vars
  updateEventFunction: EventHandlingUpdateFunction = (
    eventType: EventType,
    keyType: string,
    key: string,
    data: string,
    version: number
  ) => {
    const newStatus: Status = JSON.parse(data);

    if (newStatus) {
      this.store.dispatch(upsertOne({ status: newStatus }));
    }
  };

  // eslint-disable-next-line  @typescript-eslint/no-unused-vars
  deleteEventFunction: EventHandlingFunction = (
    eventType: EventType,
    keyType: string,
    key: string,
    data: string
  ) => {
    this.store.dispatch(removeOne({ ipaddress: key }));
  };

  constructor(
    private store: Store,
    private http: HttpClient,
    private errorService: ErrorService,
    private eventService: EventService,
    private logger: NGXLogger
  ) {
    this.eventService.registerEventHandler(
      new EventHandler(
        'Status',
        this.insertEventFunction,
        this.updateEventFunction,
        this.deleteEventFunction
      )
    );
  }

  listAllServerStatus = () => {
    const action = new ServersAction('Status', []);
    const body = JSON.stringify(action);

    this.http
      .post<Status[]>('/backend/servers/actions', body, {
        headers: defaultHeadersForJSON(),
      })
      .subscribe({
        next: (statusList) => {
          this.store.dispatch(addMany({ status: statusList }));
        },
        error: (err) => {
          if (err) {
            this.errorService.newError(
              Source.ServerStatusService,
              undefined,
              err
            );
          }
        },
      });
  };

  getServerStatus = (ipaddress: string) => {
    const action = new ServersAction('Status', []);
    const body = JSON.stringify(action);

    this.http
      .post<Status>(`/servers/${ipaddress}/actions`, body, {
        headers: defaultHeadersForJSON(),
      })
      .subscribe({
        next: (status) => {
          /*const statusUpdate: Update<Status> = {
            id: status.ipaddress,
            changes: { is_running: status.is_running }
          };
          this.store.dispatch(updateOne({status: statusUpdate}));*/

          this.store.dispatch(upsertOne({ status: status }));
        },
        error: (err) => {
          if (err) {
            this.errorService.newError(
              Source.ServerStatusService,
              undefined,
              err
            );
          }
        },
      });
  };
}
