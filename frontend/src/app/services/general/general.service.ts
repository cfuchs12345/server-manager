import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable, BehaviorSubject } from 'rxjs';
import { defaultHeadersForJSON } from '../common';
import { ConfigAction, DNSServer, SystemInformation } from './types';
import { ErrorService } from '../errors/error.service';

@Injectable({
  providedIn: 'root',
})
export class GeneralService {
  private _dnsServers = new BehaviorSubject<DNSServer[]>([]);
  private _systemDNSServers = new BehaviorSubject<DNSServer[]>([]);
  private _systemInformation= new BehaviorSubject<SystemInformation | undefined>(undefined);

  private dataStore: {
    dnsServers: DNSServer[];
    systemDNSServers: DNSServer[];
    systemInformation: SystemInformation | undefined;
  } = {
    dnsServers: [],
    systemDNSServers: [],
    systemInformation: undefined
  };

  readonly dnsServers = this._dnsServers.asObservable();
  readonly systemDNSServers = this._systemDNSServers.asObservable();
  readonly systemInformation = this._systemInformation.asObservable();

  constructor(private http: HttpClient, private errorService: ErrorService) {}

  saveDNSServer = (server: DNSServer) => {
    const body = JSON.stringify(server);

    this.http.post<boolean>('/backend/configurations/dnsservers', body, {
      headers: defaultHeadersForJSON(),
    }).subscribe({
      next: (res) => {
      },
      error: (err: any) => {
        this.errorService.newError("General-Service", undefined, err);
      },
      complete: () => {
        setTimeout(this.listDNSServers, 200);
      },
    });
  };

  deleteDNSServers = (servers: DNSServer[]) => {
    for (var i = 0; i < servers.length; i++) {
      const server = servers[i];

      this.http
        .delete<boolean>('/backend/configurations/dnsservers/' + server.ipaddress)
        .subscribe({
          next: (res) => {
          },
          error: (err: any) => {
            this.errorService.newError("General-Service",undefined, err);
          },
          complete: () => {
            if( i === servers.length ) {
              setTimeout(this.listDNSServers, 200);
            }
          },
        });
    }
  };

  listDNSServers = () => {
    this.http.get<DNSServer[]>('/backend/configurations/dnsservers').subscribe({
      next: (res) => {
        this.dataStore.dnsServers = res;
        this._dnsServers.next(this.dataStore.dnsServers.slice());
      },
      error: (err: any) => {
        this.errorService.newError("General-Service", undefined, err);
      },
      complete: () => {},
    });
  };


  listSystemDNSServers = () => {
    this.http.get<DNSServer[]>('/backend/systeminformation/dnsservers').subscribe({
      next: (res) => {
        this.dataStore.systemDNSServers = res;
        this._systemDNSServers.next(this.dataStore.systemDNSServers.slice());
      },
      error: (err: any) => {
        this.errorService.newError("General-Service", undefined, err);
      },
      complete: () => {},
    });
  };


  getSystemInformation = () => {
    this.http.get<SystemInformation>('/backend/system/information').subscribe({
      next: (res) => {
        this.dataStore.systemInformation = res;
        this._systemInformation.next(this.dataStore.systemInformation);
      },
      error: (err: any) => {
        this.errorService.newError("General-Service", undefined, err);
      },
      complete: () => {},
    });
  }
}
