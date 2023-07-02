import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable, catchError, throwError } from 'rxjs';
import { defaultHeadersForJSON } from '../common';
import { DataResult } from './types';

import { Server, ServerAction } from './types';
import { ErrorService, Source } from '../errors/error.service';

@Injectable({
  providedIn: 'root',
})
export class ServerDataService {
  constructor(private http: HttpClient, private errorService: ErrorService) {}

  queryData = (server: Server): Observable<DataResult[]> => {
    const query = new ServerAction('QueryData');

    const body = JSON.stringify(query);

    return this.http
      .post<DataResult[]>(
        '/backend/servers/' + server.ipaddress + '/actions',
        body,
        {
          headers: defaultHeadersForJSON(),
        }
      )
      .pipe(
        catchError((err) => {
          this.errorService.newError(
            Source.ServerDataService,
            server.ipaddress,
            err.error
          );
          return throwError(() => err);
        })
      );
  };
}
