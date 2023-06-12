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
import { ErrorService, Source } from '../errors/error.service';

@Injectable({
  providedIn: 'root',
})
export class ServerDataService {
  private _dataResults = new BehaviorSubject<DataResult[]>(
    []
  );

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
          this.publishDataResult(results);
        },
        error: (err: any) => {
          this.errorService.newError(Source.ServerDataService, server.ipaddress, err.error);
        },
        complete: () => {
        },
      });
  }


  private publishDataResult = (list: DataResult[]) => {
    this._dataResults.next(list);
  };

}
