import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { filter } from 'rxjs';
import { Store } from '@ngrx/store';
import { defaultHeadersForJSON } from '../common';
import { ConditionCheckResult, ServersAction } from './types';
import {
  addMany,
  removeOne,
  upsertOne,
} from 'src/app/state/actions/conditioncheckresult.action';
import { Param, ServerAction, Feature } from './types';
import { ErrorService, Source } from '../errors/error.service';
import { NGXLogger } from 'ngx-logger';
import { EventService } from '../events/event.service';
import { Event } from '../events/types';

@Injectable({
  providedIn: 'root',
})
export class ServerActionService {
  constructor(
    private store: Store,
    private http: HttpClient,
    private eventService: EventService,
    private errorService: ErrorService,
    private logger: NGXLogger
  ) {
    this.eventService.eventSubject
      .pipe(
        filter((event: Event) => {
          return event.object_type === 'ConditionCheckResult';
        })
      )
      .subscribe((event: Event) => {
        if (event.event_type === 'Insert' || event.event_type === 'Update') {
          const result: ConditionCheckResult = JSON.parse(event.value);

          this.store.dispatch(upsertOne({ result: result }));
        } else if (event.event_type === 'Delete') {
          this.store.dispatch(removeOne({ ipaddress: event.key }));
        }
      });
  }

  listActionCheckResults = () => {
    const action = new ServersAction('ActionConditionCheck', []);
    const body = JSON.stringify(action);

    const subscription = this.http
      .post<ConditionCheckResult[]>('/backend/servers/actions', body, {
        headers: defaultHeadersForJSON(),
      })
      .subscribe({
        next: (results) => {
          this.store.dispatch(addMany({ results: results }));
        },
        error: (err) => {
          this.logger.error(err);

          this.errorService.newError(
            Source.ServerActionService,
            undefined,
            err !== undefined ? err : err
          );
        },
        complete: () => {
          subscription.unsubscribe();
        },
      });
  };

  executeAction = (
    feature_id: string,
    action_id: string,
    ipaddress: string,
    action_params: string | undefined = undefined
  ) => {
    const query = new ServerAction('ExecuteFeatureAction');
    query.params.push(new Param('feature_id', feature_id));
    query.params.push(new Param('action_id', action_id));
    if (action_params) {
      query.params.push(new Param('action_params', action_params));
    }

    const body = JSON.stringify(query);

    const subscription = this.http
      .post<Feature[]>('/backend/servers/' + ipaddress + '/actions', body, {
        headers: defaultHeadersForJSON(),
      })
      .subscribe({
        error: (err) => {
          this.errorService.newError(
            Source.ServerActionService,
            ipaddress,
            err
          );
        },
        complete: () => {
          subscription.unsubscribe();
        },
      });
  };
}
