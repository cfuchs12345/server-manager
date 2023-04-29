import { Injectable } from '@angular/core';
import { Feature, HostInformation, NetworksAction, Param, ServerAction, ServerFeature, ServersAction } from "./types";
import { defaultHeadersForJSON } from '../common';
import { ErrorService } from '../errors/error.service';
import { HttpClient } from '@angular/common/http';
import { BehaviorSubject } from 'rxjs';
import { DNSServer } from '../general/types';

@Injectable({
  providedIn: 'root',
})
export class ServerDiscoveryService {
  private _discoveredServers = new BehaviorSubject<HostInformation[]>([]);
  private _discoveredServerFeatures = new BehaviorSubject<ServerFeature[]>([]);

  readonly discoveredServers = this._discoveredServers.asObservable();
  readonly discoveredServerFeatures = this._discoveredServerFeatures.asObservable();

  private dataStore: {
    discoveredServers: HostInformation[];
    discoveredServerFeatures: ServerFeature[];
  } = {
    discoveredServers: [],
    discoveredServerFeatures: [],
  };



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
          this.errorService.newError(this, ipaddress, err.message);
        },
        complete: () => {},
      });
  };

  autoDiscoverServers = (
    network: string,
    dnsLookup: boolean
  ) => {
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

          this.dataStore.discoveredServers = relevant_servers;
          this.publishDiscoveredServers();
        },
        error: (err: any) => {
          this.errorService.newError(this, undefined, err.message);

          this.resetDiscoveredServers();
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
          this.dataStore.discoveredServerFeatures.splice(
            0,
            this.dataStore.discoveredServerFeatures.length
          );
          this.dataStore.discoveredServerFeatures.push(...serverFeatures);
          this.publishDiscoveredServerFeatures();
        },
        error: (err: any) => {
          this.errorService.newError(this, undefined, err.message);
        },
        complete: () => {},
      });
  };

  resetDiscoveredServerFeatures = () => {
    this.dataStore.discoveredServerFeatures = [];
    this.publishDiscoveredServerFeatures();
  };


  resetDiscoveredServers = () => {
    this.dataStore.discoveredServers.splice(
      0,
      this.dataStore.discoveredServers.length
    );
    this.publishDiscoveredServers();
  };


  private publishDiscoveredServers = () => {
    this._discoveredServers.next(
      Object.assign({}, this.dataStore).discoveredServers
    );
  };

  private publishDiscoveredServerFeatures = () => {
    this._discoveredServerFeatures.next(
      Object.assign({}, this.dataStore).discoveredServerFeatures
    );
  };
}
