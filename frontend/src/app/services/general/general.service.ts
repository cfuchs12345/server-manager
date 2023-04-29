import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable, BehaviorSubject } from 'rxjs';
import { defaultHeadersForJSON } from '../common';
import { ConfigAction, DNSServer } from './types';
import { ErrorService } from '../errors/error.service';

@Injectable({
  providedIn: 'root',
})
export class GeneralService {
  private _dnsServers = new BehaviorSubject<DNSServer[]>([]);

  private dataStore: {
    dnsServers: DNSServer[];
  } = {
    dnsServers: [],
  };

  readonly dnsServers = this._dnsServers.asObservable();

  constructor(private http: HttpClient, private errorService: ErrorService) {}

  saveDNSServer = (server: DNSServer) => {
    const body = JSON.stringify(server);

    this.http.post<boolean>('/backend/configurations/dnsservers', body, {
      headers: defaultHeadersForJSON(),
    }).subscribe({
      next: (res) => {
        this.listDNSServers();
      },
      error: (err: any) => {
        this.errorService.newError(this, undefined, err.message);
      },
      complete: () => {},
    });
  };

  deleteDNSServers = (servers: DNSServer[]) => {
    for (var i = 0; i < servers.length; i++) {
      const server = servers[i];

      this.http
        .delete<boolean>('/backend/configurations/dnsservers/' + server.ipaddress)
        .subscribe({
          next: (res) => {
            if( i === servers.length ) {
              setTimeout(this.listDNSServers, 200);
            }
          },
          error: (err: any) => {
            this.errorService.newError(this,undefined, err.message);
          },
          complete: () => {},
        });
    }
  };

  listDNSServers = () => {
    this.http.get<DNSServer[]>('/backend/configurations/dnsservers').subscribe({
      next: (res) => {
        this.dataStore.dnsServers = res;
        this._dnsServers.next(this.dataStore.dnsServers);
      },
      error: (err: any) => {
        this.errorService.newError(this, undefined, err.message);
      },
      complete: () => {},
    });
  };
}
