import { Injectable } from '@angular/core';
import { HttpClient, HttpParams } from '@angular/common/http';
import { BehaviorSubject } from 'rxjs';
import { ErrorService } from '../errors/error.service';
import { Server } from '../servers/types';
import { MonitoringData, MonitoringSeriesData } from './types';

@Injectable({
  providedIn: 'root',
})
export class MonitoringService {
  private _data = new BehaviorSubject<MonitoringData | undefined>(
    undefined
  );
  private _monitoringSeriesData = new BehaviorSubject<MonitoringSeriesData | undefined>(
    undefined
  );

  readonly data = this._data.asObservable();
  readonly monitoringIds = this._monitoringSeriesData.asObservable();

  constructor(private http: HttpClient, private errorService: ErrorService) {}


  getMonitoringIds = (server: Server) => {
    const options =
    server !== undefined
      ? { params: new HttpParams().set('ipaddress', server.ipaddress) }
      : {};

      this.http
      .get<string[]>('/backend/monitoring/ids', options)
      .subscribe({
        next: (ids) => {
          setTimeout( () => {this.publishMonitoringSeriesData(new MonitoringSeriesData(server.ipaddress, ids))}, 100);
        },
        error: (err: any) => {
          this.errorService.newError(
            'Monitoring-Service',
            undefined,
            err.message
          );
        },
        complete: () => {
        },
      });
  }

  loadMonitoringData = (server: Server, series_id: string) => {
    const options =
      server !== undefined
        ? { params: new HttpParams().set('ipaddress', server.ipaddress).set('series_id', series_id) }
        : {};

    this.http
      .get<string>('/backend/monitoring/data', options)
      .subscribe({
        next: (response) => {
          setTimeout( () => {this.publisMonitoringhData(new MonitoringData(server.ipaddress, response))}, 100);
        },
        error: (err: any) => {
          this.errorService.newError(
            'Monitoring-Service',
            undefined,
            err.message
          );
        },
        complete: () => {
        },
      });
  };

  private publisMonitoringhData = (data: MonitoringData) => {
    if( data !== undefined ){
      this._data.next( data );
    }
  };

  private publishMonitoringSeriesData = ( data: MonitoringSeriesData) => {
      if( data !== undefined ) {
       this._monitoringSeriesData.next( data);
      }

  }
}
