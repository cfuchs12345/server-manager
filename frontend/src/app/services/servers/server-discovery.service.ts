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
import { BehaviorSubject } from 'rxjs';

@Injectable({
  providedIn: 'root',
})
export class ServerDiscoveryService {
  private _discoveredServers = new BehaviorSubject<HostInformation[]>([]);
  private _discoveredServerFeatures = new BehaviorSubject<ServerFeature[]>([]);

  readonly discoveredServers = this._discoveredServers.asObservable();
  readonly discoveredServerFeatures =
    this._discoveredServerFeatures.asObservable();

  constructor(private http: HttpClient, private errorService: ErrorService) {}

  scanFeature = (ipaddress: string) => {
    const query = new ServerAction('FeatureScan');

    const body = JSON.stringify(query);

    this.http
      .post<Feature[]>('/backend/servers/' + ipaddress + '/actions', body, {
        headers: defaultHeadersForJSON(),
      })
      .subscribe({
        next: (foundFeature) => {
          var f: Feature[] = foundFeature;
        },
        error: (err: any) => {
          this.errorService.newError(
            Source.ServerDiscoveryService,
            ipaddress,
            err
          );
        },
        complete: () => {},
      });
  };

  autoDiscoverServers = (network: string, dnsLookup: boolean) => {
    const params = [
      new Param('network', network),
      new Param('lookup_names', dnsLookup ? 'true' : 'false'),
    ];
    const query = new NetworksAction('AutoDiscover', params);

    const body = JSON.stringify(query);

    this.http
      .post<HostInformation[]>('/backend/networks/actions', body, {
        headers: defaultHeadersForJSON(),
      })
      .subscribe({
        next: (foundServers) => {
          const relevant_servers = foundServers.filter(
            (filter) =>
              filter.is_running || (filter.dnsname && filter.dnsname !== '')
          );

          this.publishDiscoveredServers(relevant_servers);
        },
        error: (err: any) => {
          this.errorService.newError(
            Source.ServerDiscoveryService,
            undefined,
            err
          );
        },
        complete: () => {},
      });
  };

  scanFeatureOfAllServers = () => {
    const query = new ServersAction('FeatureScan');

    const body = JSON.stringify(query);

    this.http
      .post<ServerFeature[]>('/backend/servers/actions', body, {
        headers: defaultHeadersForJSON(),
      })
      .subscribe({
        next: (serverFeatures) => {
          this.publishDiscoveredServerFeatures(serverFeatures);
        },
        error: (err: any) => {
          this.errorService.newError(
            Source.ServerDiscoveryService,
            undefined,
            err
          );
        },
        complete: () => {},
      });
  };

  private publishDiscoveredServers = (list: HostInformation[]) => {
    this._discoveredServers.next(list);
  };

  private publishDiscoveredServerFeatures = (list: ServerFeature[]) => {
    this._discoveredServerFeatures.next(list);
  };
}
