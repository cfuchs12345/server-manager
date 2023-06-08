import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { BehaviorSubject } from 'rxjs';
import { defaultHeadersForJSON } from '../common';
import {
  ConditionCheckResult,
  ServersAction,
} from './types';

import { Server, Param, ServerAction, Feature } from './types';
import { ErrorService } from '../errors/error.service';
import { Action } from '../plugins/types';

@Injectable({
  providedIn: 'root',
})
export class ServerActionService {
  private _actionConditionChecks = new BehaviorSubject<ConditionCheckResult[]>(
    []
  );
  readonly actionConditionChecks = this._actionConditionChecks.asObservable();

  private dataStore: {
    actionConditionChecks: ConditionCheckResult[];
  } = {
    actionConditionChecks: [],
  };

  constructor(private http: HttpClient, private errorService: ErrorService) {}

  listActionCheckResults = () => {
    const action = new ServersAction('ActionConditionCheck', []);
    const body = JSON.stringify(action);

    this.http
      .post<ConditionCheckResult[]>('/backend/servers/actions', body, {
        headers: defaultHeadersForJSON(),
      })
      .subscribe({
        next: (results) => {
          this.dataStore.actionConditionChecks.splice(
            0,
            this.dataStore.actionConditionChecks.length
          );
          this.dataStore.actionConditionChecks.push(...results);
          this.publishActionCheckResult();
        },
        error: (err: any) => {
          this.errorService.newError('Action-Service', undefined, err !== undefined ? err: err);
        },
        complete: () => {},
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

    this.http
      .post<Feature[]>('/backend/servers/' + ipaddress + '/actions', body, {
        headers: defaultHeadersForJSON(),
      })
      .subscribe({
        next: (result) => {},
        error: (err: any) => {
          this.errorService.newError('Action-Service', ipaddress, err);
        },
        complete: () => {},
      });
  };

  private publishActionCheckResult = () => {
    this._actionConditionChecks.next(
      this.dataStore.actionConditionChecks.slice()
    );
  };
}
