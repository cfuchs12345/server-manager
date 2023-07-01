import { Injectable } from '@angular/core';
import { HttpClient, HttpParams } from '@angular/common/http';
import { Observable, map, catchError, throwError } from 'rxjs';
import { ErrorService, Source } from '../errors/error.service';
import { Server } from '../servers/types';
import { TimeSeriesIds, TimeSeriesResponse } from './types';

@Injectable({
  providedIn: 'root',
})
export class MonitoringService {
  constructor(private http: HttpClient, private errorService: ErrorService) {}

  getMonitoringIds = (server: Server): Observable<TimeSeriesIds> => {
    const options = server
      ? { params: new HttpParams().set('ipaddress', server.ipaddress) }
      : {};

    return this.http.get<string[]>('/backend/monitoring/ids', options).pipe(
      map( (ids) => {
        return new TimeSeriesIds(server.ipaddress, ids);
      }),
      catchError((err) => {
        this.errorService.newError(Source.MonitoringService, undefined, err);
        return throwError(() => err);
      })
    );
  };

  loadMonitoringData = (
    server: Server,
    series_id: string
  ): Observable<TimeSeriesResponse> => {
    const options = server
      ? {
          params: new HttpParams()
            .set('ipaddress', server.ipaddress)
            .set('series_id', series_id),
        }
      : {};

    return this.http
      .get<TimeSeriesResponse>('/backend/monitoring/data', options)
      .pipe(
        catchError((err) => {
          this.errorService.newError(Source.MonitoringService, undefined, err);
          return throwError(() => err);
        })
      );
  };
}
