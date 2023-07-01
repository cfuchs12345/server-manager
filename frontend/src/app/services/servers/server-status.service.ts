import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { filter } from 'rxjs';
import { defaultHeadersForJSON } from '../common';
import { ServersAction } from './types';

import { Status } from './types';
import { ErrorService, Source } from '../errors/error.service';
import { EventService } from '../events/event.service';
import { Event } from '../events/types';
import { NGXLogger } from 'ngx-logger';
import { addMany, upsertOne } from 'src/app/state/actions/status.action';
import { Store } from '@ngrx/store';
import { Update } from '@ngrx/entity';

@Injectable({
  providedIn: 'root',
})
export class ServerStatusService {
  constructor(private store: Store, private http: HttpClient, private errorService: ErrorService, private eventService: EventService, private logger: NGXLogger) {

    this.eventService.eventSubject.pipe(
      filter( (event: Event) => { return event.object_type === 'Status'; })
    )
    .subscribe( (event: Event) => {
      const newStatus: Status = JSON.parse(event.value);

      if( newStatus ) {
        this.store.dispatch(upsertOne({status: newStatus}));
      }
    });
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
          this.store.dispatch(addMany({status: statusList}));
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

          this.store.dispatch(upsertOne({status: status}));
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
  }

}
