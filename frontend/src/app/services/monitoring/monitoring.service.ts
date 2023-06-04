import { Injectable } from '@angular/core';
import { HttpClient, HttpParams } from '@angular/common/http';
import { BehaviorSubject } from 'rxjs';
import { ErrorService } from '../errors/error.service';
import { Server } from '../servers/types';

@Injectable({
  providedIn: 'root',
})
export class MonitoringService {
  private _data = new BehaviorSubject<any | undefined>(
    undefined
  );
  private _monitoringIds = new BehaviorSubject<string[] | undefined>(
    undefined
  );

  readonly data = this._data.asObservable();
  readonly monitoringIds = this._monitoringIds.asObservable();

  constructor(private http: HttpClient, private errorService: ErrorService) {}


  getMonitoringNames = (server: Server) => {
    const options =
    server !== undefined
      ? { params: new HttpParams().set('ipaddress', server.ipaddress) }
      : {};

      this.http
      .get<string[]>('/backend/monitoring/ids', options)
      .subscribe({
        next: (ids) => {
          this.publishIds(ids);
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
          setTimeout( () => {this.publishData(response)}, 10);
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

  private publishData = (response: string | undefined) => {
    if( response !== undefined ){
      this._data.next( JSON.parse(response));
    }
  };

  private publishIds = ( ids: string[]) => {
      if( ids !== undefined ) {
       this._monitoringIds.next( ids);
      }

  }
}
