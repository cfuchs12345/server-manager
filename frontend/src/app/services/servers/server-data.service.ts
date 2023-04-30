import { Injectable } from '@angular/core';
import {
  HttpClient,
  HttpErrorResponse,
} from '@angular/common/http';
import { BehaviorSubject } from 'rxjs';
import { defaultHeadersForJSON } from '../common';
import { DataResult } from './types';

import {
  Server,
  Param,
  ServerAction,
  Feature,
  ServerFeature,
} from './types';
import { ErrorService } from '../errors/error.service';

@Injectable({
  providedIn: 'root',
})
export class ServerDataService {
  private _dataResults = new BehaviorSubject<Map<String, DataResult>>(
    new Map()
  );
  private dataStore: {
    dataResults: Map<string, DataResult>;
  } = {
    dataResults: new Map(),
  };

  readonly dataResults = this._dataResults.asObservable();

  constructor(private http: HttpClient, private errorService: ErrorService) {}


  queryData(server: Server) {
    const query = new ServerAction('QueryData');

    const body = JSON.stringify(query);

    this.http
      .post<string[]>(
        '/backend/servers/' + server.ipaddress + '/actions',
        body,
        {
          headers: defaultHeadersForJSON(),
        }
      )
      .subscribe({
        next: (results) => {
          this.dataStore.dataResults.set(
            server.ipaddress,
            new DataResult(new Date(), results)
          );
          this.publishDataResult();
        },
        error: (err: HttpErrorResponse) => {
          this.errorService.newError("Data-Service", server.ipaddress, err.message);
        },
        complete: () => {},
      });
  }


  private publishDataResult = () => {
    this._dataResults.next(Object.assign({}, this.dataStore).dataResults);
  };

}
