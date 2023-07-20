import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable, of } from 'rxjs';
import { Store } from '@ngrx/store';
import { defaultHeadersForJSON } from '../common';
import { ConditionCheckResult, ServersAction } from './types';
import {
  removeOne,
  upsertOne,
} from 'src/app/state/conditioncheckresult/conditioncheckresult.actions';
import { Param, ServerAction, Feature } from './types';
import { ErrorService } from '../errors/error.service';
import { NGXLogger } from 'ngx-logger';
import {
  EventHandler,
  EventHandlingFunction,
  EventHandlingGetObjectFunction,
  EventHandlingUpdateFunction,
  EventType,
} from '../events/types';
import { EventService } from '../events/event.service';

@Injectable({
  providedIn: 'root',
})
export class ServerActionService {
  // eslint-disable-next-line  @typescript-eslint/no-unused-vars
  insertEventFunction: EventHandlingFunction<ConditionCheckResult> = (
    eventType: EventType,
    keyType: string,
    key: string,
    data: string,
    object: ConditionCheckResult
  ) => {
    this.store.dispatch(upsertOne({ result: object }));
  };

  updateEventFunction: EventHandlingUpdateFunction<ConditionCheckResult> = (
    eventType: EventType,
    keyType: string,
    key: string,
    data: string,
    version: number,
    object: ConditionCheckResult
  ) => {
    this.store.dispatch(upsertOne({ result: object }));
  };

  // eslint-disable-next-line  @typescript-eslint/no-unused-vars
  deleteEventFunction: EventHandlingFunction<ConditionCheckResult> = (
    eventType: EventType,
    key_name: string,
    key: string,
    data: string // eslint-disable-line @typescript-eslint/no-unused-vars
  ) => {
    this.store.dispatch(removeOne({ key: key }));
  };

  getObjectFunction: EventHandlingGetObjectFunction<ConditionCheckResult> = (
    key_name: string,
    key: string,
    value: string
  ): Observable<ConditionCheckResult> => {
    const result: ConditionCheckResult = JSON.parse(value);
    return of(result);
  };

  constructor(
    private store: Store,
    private http: HttpClient,
    private eventService: EventService,
    private errorService: ErrorService,
    private logger: NGXLogger
  ) {
    this.eventService.registerEventHandler(
      new EventHandler(
        'ConditionCheckResult',
        this.insertEventFunction,
        this.updateEventFunction,
        this.deleteEventFunction,
        this.getObjectFunction
      )
    );
  }

  listActionCheckResults = (): Observable<ConditionCheckResult[]> => {
    const action = new ServersAction('ActionConditionCheck', []);
    const body = JSON.stringify(action);

    return this.http.post<ConditionCheckResult[]>(
      '/backend/servers/actions',
      body,
      {
        headers: defaultHeadersForJSON(),
      }
    );
  };

  executeAction = (
    feature_id: string,
    action_id: string,
    ipaddress: string,
    action_params: string | undefined = undefined
  ): Observable<Feature[]> => {
    const query = new ServerAction('ExecuteFeatureAction');
    query.params.push(new Param('feature_id', feature_id));
    query.params.push(new Param('action_id', action_id));
    if (action_params) {
      query.params.push(new Param('action_params', action_params));
    }

    const body = JSON.stringify(query);

    return this.http.post<Feature[]>(
      '/backend/servers/' + ipaddress + '/actions',
      body,
      {
        headers: defaultHeadersForJSON(),
      }
    );
  };
}
