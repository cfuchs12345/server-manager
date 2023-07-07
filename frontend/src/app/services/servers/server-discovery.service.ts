import { Injectable } from '@angular/core';
import {
  Feature,
  HostInformation,
  NetworksAction,
  Param,
  ServerAction,
  ServerFeature,
  ServersAction,
} from './types';
import { defaultHeadersForJSON } from '../common';
import { ErrorService, Source } from '../errors/error.service';
import { HttpClient } from '@angular/common/http';
import { Observable, catchError,  throwError } from 'rxjs';

@Injectable({
  providedIn: 'root',
})
export class ServerDiscoveryService {
  constructor(private http: HttpClient, private errorService: ErrorService) {}

  scanFeature = (ipaddress: string): Observable<Feature[]> => {
    const query = new ServerAction('FeatureScan');

    const body = JSON.stringify(query);

    return this.http
      .post<Feature[]>('/backend/servers/' + ipaddress + '/actions', body, {
        headers: defaultHeadersForJSON(),
      })
      .pipe(
        catchError((err) => {
          this.errorService.newError(
            Source.ServerDiscoveryService,
            ipaddress,
            err
          );
          return throwError(() => err);
        })
      );
  };

  autoDiscoverServers = (network: string, dnsLookup: boolean): Observable<HostInformation[]> => {
    const params = [
      new Param('network', network),
      new Param('lookup_names', dnsLookup ? 'true' : 'false'),
    ];
    const query = new NetworksAction('AutoDiscover', params);

    const body = JSON.stringify(query);

    return this.http
      .post<HostInformation[]>('/backend/networks/actions', body, {
        headers: defaultHeadersForJSON(),
      })
      .pipe(
        catchError((err) => {
          this.errorService.newError(
            Source.ServerDiscoveryService,
            undefined,
            err
          );
          return throwError(() => err);
        })
      );
  };

  scanFeatureOfAllServers = (): Observable<ServerFeature[]> => {
    const query = new ServersAction('FeatureScan');

    const body = JSON.stringify(query);

    return this.http
      .post<ServerFeature[]>('/backend/servers/actions', body, {
        headers: defaultHeadersForJSON(),
      })
      .pipe(
        catchError((err) => {
          this.errorService.newError(
            Source.ServerDiscoveryService,
            undefined,
            err
          );
          return throwError(() => err);
        })
      );
  };
}
