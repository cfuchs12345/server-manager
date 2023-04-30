import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { BehaviorSubject } from 'rxjs';
import { defaultHeadersForJSON } from '../common';
import { ConditionCheck, ConditionCheckResult, newConditionCheckResultFromCheck } from './types';

import { Server, Param, ServerAction, Feature } from './types';
import { ErrorService } from '../errors/error.service';
import { Action } from '../plugins/types';

@Injectable({
  providedIn: 'root',
})
export class ServerActionService {
  private monitoredActions: ConditionCheck[] = [];

  private _conditionChecks = new BehaviorSubject<ConditionCheckResult[]>([]);
  readonly conditionChecks = this._conditionChecks.asObservable();


  private dataStore: {
    conditionChecks: ConditionCheckResult[],
  } = {
    conditionChecks: [],
  };

  constructor(private http: HttpClient, private errorService: ErrorService) {
    console.log("ServerActionService instanciated");

    setInterval(this.checkFeatureActionConditionsMet, 20000);
  }


  executeAction = (feature_id: string, action_id: string, server: Server) => {
    const query = new ServerAction('ExecuteFeatureAction');
    query.params.push(new Param('feature_id', feature_id));
    query.params.push(new Param('action_id', action_id));

    const body = JSON.stringify(query);

    this.http
      .post<Feature[]>(
        '/backend/servers/' + server.ipaddress + '/actions',
        body,
        {
          headers: defaultHeadersForJSON(),
        }
      )
      .subscribe({
        next: (result) => {},
        error: (err: any) => {
          this.errorService.newError("Action-Service", server.ipaddress, err.message);
        },
        complete: () => {},
      });
  };

  registerFeatureActionOfServerForCheck = (server: Server, feature: Feature, action: Action) => {
    const found = this.monitoredActions.find( c => c.ipaddress === server.ipaddress && c.feature_id === feature.id && c.action_id === action.id);

    if( found ) {
      return;
    }
    this.monitoredActions.push(new ConditionCheck(server.ipaddress, feature.id, action.id));
  }


  private checkFeatureActionConditionsMet = () => {
    for( const check of this.monitoredActions ) {
      this.check( check);
    }
  }

  private check = (check: ConditionCheck) => {
    const query = new ServerAction('ActionConditionCheck');
    query.params.push(new Param('feature_id',check.feature_id));
    query.params.push(new Param('action_id', check.action_id));


    const body = JSON.stringify(query);

    this.http
      .post<boolean>(
        '/backend/servers/' + check.ipaddress + '/actions',
        body,
        {
          headers: defaultHeadersForJSON(),
        }
      )
      .subscribe({
        next: (result) => {
          const updated = this.addCheckResultToDatastore(check, result);

          if( updated ) {
            this.publishDataCheckResult(this.dataStore.conditionChecks);
          }
        },
        error: (err: any) => {
          this.errorService.newError("Action-Service", check.ipaddress, err.message);
        },
        complete: () => {},
      });
  };

  private publishDataCheckResult = (toPublish: ConditionCheckResult[] ) => {
    this._conditionChecks.next(toPublish);
  }


  private addCheckResultToDatastore = (check: ConditionCheck, result: boolean): boolean => {
    const newResult = newConditionCheckResultFromCheck(check, result);

    var found = this.dataStore.conditionChecks.find( c => c.ipaddress === check.ipaddress && c.feature_id === check.feature_id && c.action_id === check.action_id);

    if( found ) {
      if( found.result !== result) {
        found.result = result;
        return true;
      }
    }
    else {
      this.dataStore.conditionChecks.push(newResult);
      return true;
    }
    return false;
  }

}
