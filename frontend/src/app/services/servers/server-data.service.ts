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
  ServerAction,
} from './types';
import { ErrorService } from '../errors/error.service';

@Injectable({
  providedIn: 'root',
})
export class ServerDataService {
  private _dataResults = new BehaviorSubject<DataResult[]>(
    []
  );
  private dataStore: {
    dataResults: DataResult[];
  } = {
    dataResults: [],
  };

  readonly dataResults = this._dataResults.asObservable();

  constructor(private http: HttpClient, private errorService: ErrorService) {}


  queryData(server: Server) {
    const query = new ServerAction('QueryData');

    const body = JSON.stringify(query);

    this.http
      .post<DataResult[]>(
        '/backend/servers/' + server.ipaddress + '/actions',
        body,
        {
          headers: defaultHeadersForJSON(),
        }
      )
      .subscribe({
        next: (results) => {
          this.dataStore.dataResults.splice(0, this.dataStore.dataResults.length);
          this.dataStore.dataResults.push(...results);

          this.publishDataResult();
        },
        error: (err: HttpErrorResponse) => {
          this.errorService.newError("Data-Service", server.ipaddress, err.error);
        },
        complete: () => {},
      });
  }


  private publishDataResult = () => {
    this._dataResults.next(this.dataStore.dataResults.slice(0, this.dataStore.dataResults.length));
  };

}
