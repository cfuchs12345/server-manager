import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { defaultHeadersForJSON } from '../common';
import { ServersAction } from './types';

import { Status } from './types';
import { ErrorService, Source } from '../errors/error.service';
import {
  EventHandler,
  EventHandlingGetObjectFunction,
  EventHandlingFunction,
  EventHandlingUpdateFunction,
} from '../events/types';
import {
  EventService,
} from '../events/event.service';
import { EventType } from '../events/types';
import { NGXLogger } from 'ngx-logger';
import {
  removeOne,
  upsertOne,
} from 'src/app/state/status/status.actions';
import { Store } from '@ngrx/store';
import { Observable, catchError, of, take, throwError } from 'rxjs';

@Injectable({
  providedIn: 'root',
})
export class ServerStatusService {
  // eslint-disable-next-line  @typescript-eslint/no-unused-vars
  insertEventFunction: EventHandlingFunction<Status> = (
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
  updateEventFunction: EventHandlingUpdateFunction<Status> = (
    eventType: EventType,
    keyType: string,
    key: string,
    data: string,
    version: number,
    object: Status
  ) => {
    if (object) {
      this.store.dispatch(upsertOne({ status: object }));
    }
  };

  // eslint-disable-next-line  @typescript-eslint/no-unused-vars
  deleteEventFunction: EventHandlingFunction<Status> = (
    eventType: EventType,
    keyType: string,
    key: string,
    data: string, // eslint-disable-line @typescript-eslint/no-unused-vars
    object: Status // eslint-disable-line @typescript-eslint/no-unused-vars
  ) => {
    this.store.dispatch(removeOne({ ipaddress: key }));
  };

  getObjectFunction: EventHandlingGetObjectFunction<Status> = (
    key_name: string,
    key: string,
    value: string
  ): Observable<Status> => {
    const status: Status = JSON.parse(value);
    return of(status);
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
        this.deleteEventFunction,
        this.getObjectFunction
      )
    );
  }

  listAllServerStatus = (): Observable<Status[]> => {
    const action = new ServersAction('Status', []);
    const body = JSON.stringify(action);

    return this.http.post<Status[]>('/backend/servers/actions', body, {
      headers: defaultHeadersForJSON(),
    });
  };

  getServerStatus = (ipaddress: string): Observable<Status> => {
    const action = new ServersAction('Status', []);
    const body = JSON.stringify(action);

    return this.http
      .post<Status>(`/backend/servers/${ipaddress}/actions`, body, {
        headers: defaultHeadersForJSON(),
      })
      .pipe(
        take(1),
        catchError((err) => {
          if (err) {
            this.errorService.newError(
              Source.ServerStatusService,
              undefined,
              err
            );
          }
          return throwError(() => err);
        })
      );
  };
}
