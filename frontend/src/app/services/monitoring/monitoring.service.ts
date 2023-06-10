import { Injectable } from '@angular/core';
import { HttpClient, HttpParams } from '@angular/common/http';
import { BehaviorSubject } from 'rxjs';
import { ErrorService, Source } from '../errors/error.service';
import { Server } from '../servers/types';
import { TimeSeriesIds, TimeSeriesResponse } from './types';

@Injectable({
  providedIn: 'root',
})
export class MonitoringService {
  private _data = new BehaviorSubject<TimeSeriesResponse | undefined>(
    undefined
  );
  private _monitoringSeriesData = new BehaviorSubject<
    TimeSeriesIds | undefined
  >(undefined);

  readonly data = this._data.asObservable();
  readonly monitoringIds = this._monitoringSeriesData.asObservable();

  constructor(private http: HttpClient, private errorService: ErrorService) {}

  getMonitoringIds = (server: Server) => {
    const options =
      server !== undefined
        ? { params: new HttpParams().set('ipaddress', server.ipaddress) }
        : {};

    this.http.get<string[]>('/backend/monitoring/ids', options).subscribe({
      next: (ids) => {
        setTimeout(() => {
          this.publishMonitoringSeriesData(
            new TimeSeriesIds(server.ipaddress, ids)
          );
        }, 100);
      },
      error: (err: any) => {
        this.errorService.newError(
          Source.MonitoringService,
          undefined,
          err
        );
      },
    });
  };

  loadMonitoringData = (server: Server, series_id: string) => {
    const options =
      server !== undefined
        ? {
            params: new HttpParams()
              .set('ipaddress', server.ipaddress)
              .set('series_id', series_id),
          }
        : {};

    this.http
      .get<TimeSeriesResponse>('/backend/monitoring/data', options)
      .subscribe({
        next: (response) => {
          setTimeout(() => {
            this.publisMonitoringhData(response);
          }, 100);
        },
        error: (err: any) => {
          this.errorService.newError(
            Source.MonitoringService,
            undefined,
            err
          );
        },
      });
  };

  private publisMonitoringhData = (data: TimeSeriesResponse) => {
    if (data !== undefined) {
      this._data.next(data);
    }
  };

  private publishMonitoringSeriesData = (data: TimeSeriesIds) => {
    if (data !== undefined) {
      this._monitoringSeriesData.next(data);
    }
  };
}
