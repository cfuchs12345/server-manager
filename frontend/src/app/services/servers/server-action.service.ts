import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { BehaviorSubject } from 'rxjs';
import { defaultHeadersForJSON } from '../common';
import { ConditionCheck, ConditionCheckResult, ServersAction, newConditionCheckResultFromCheck } from './types';

import { Server, Param, ServerAction, Feature } from './types';
import { ErrorService } from '../errors/error.service';
import { Action } from '../plugins/types';

@Injectable({
  providedIn: 'root',
})
export class ServerActionService {
  private _conditionChecks = new BehaviorSubject<ConditionCheckResult[]>([]);
  readonly conditionChecks = this._conditionChecks.asObservable();


  private dataStore: {
    conditionChecks: ConditionCheckResult[],
  } = {
    conditionChecks: [],
  };

  constructor(private http: HttpClient, private errorService: ErrorService) {
  }

  listConditionCheckResults = () => {
    const action = new ServersAction('ActionConditionCheck', []);
    const body = JSON.stringify(action);

    this.http
      .post<ConditionCheckResult[]>('/backend/servers/actions', body, {
        headers: defaultHeadersForJSON(),
      })
      .subscribe({
        next: (results) => {
          this.dataStore.conditionChecks.splice(
            0,
            this.dataStore.conditionChecks.length
          );
          this.dataStore.conditionChecks.push(...results);
          this.publishDataCheckResult();
        },
        error: (err: any) => {
          this.errorService.newError("Status-Service", undefined, err.message);
        },
        complete: () => {},
      });
  };




  executeAction = (feature_id: string, action_id: string, ipaddress: string, action_params: string | undefined = undefined) => {
    const query = new ServerAction('ExecuteFeatureAction');
    query.params.push(new Param('feature_id', feature_id));
    query.params.push(new Param('action_id', action_id));
    if( action_params ) {
      action_params = action_params.replace("=", "|");

      console.log(action_params);
      query.params.push(new Param('action_params', action_params));
    }

    const body = JSON.stringify(query);
    console.log(body);

    this.http
      .post<Feature[]>(
        '/backend/servers/' + ipaddress + '/actions',
        body,
        {
          headers: defaultHeadersForJSON(),
        }
      )
      .subscribe({
        next: (result) => {},
        error: (err: any) => {
          this.errorService.newError("Action-Service", ipaddress, err.message);
        },
        complete: () => {},
      });
  };



  private publishDataCheckResult = () => {
    this._conditionChecks.next(this.dataStore.conditionChecks.slice(0, this.dataStore.conditionChecks.length));
  }
}
