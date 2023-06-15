import { Injectable } from '@angular/core';
import { HttpClient, HttpHeaders } from '@angular/common/http';
import {
  Observable,
  BehaviorSubject,
  catchError,
  throwError,
  mergeMap,
} from 'rxjs';
import { defaultHeadersForJSON } from '../common';
import { Configuration, DNSServer, SystemInformation } from './types';
import { ErrorService, Source } from '../errors/error.service';
import { EncryptionService } from '../encryption/encryption.service';
import { OneTimeKey } from '../auth/types';
import { NGXLogger } from 'ngx-logger';

@Injectable({
  providedIn: 'root',
})
export class GeneralService {
  private _configuration = new BehaviorSubject<Configuration | undefined>(
    undefined
  );
  private _dnsServers = new BehaviorSubject<DNSServer[]>([]);
  private _systemDNSServers = new BehaviorSubject<DNSServer[]>([]);
  private _systemInformation = new BehaviorSubject<
    SystemInformation | undefined
  >(undefined);

  private dataStore: {
    configuration: Configuration | undefined;
    dnsServers: DNSServer[];
    systemDNSServers: DNSServer[];
    systemInformation: SystemInformation | undefined;
  } = {
    configuration: undefined,
    dnsServers: [],
    systemDNSServers: [],
    systemInformation: undefined,
  };
  readonly configuration = this._configuration.asObservable();
  readonly dnsServers = this._dnsServers.asObservable();
  readonly systemDNSServers = this._systemDNSServers.asObservable();
  readonly systemInformation = this._systemInformation.asObservable();

  constructor(
    private http: HttpClient,
    private errorService: ErrorService,
    private encryptionService: EncryptionService,
    private logger: NGXLogger
  ) {}

  saveDNSServer = (server: DNSServer) => {
    const body = JSON.stringify(server);

    this.http
      .post<boolean>('/backend/configurations/dnsservers', body, {
        headers: defaultHeadersForJSON(),
      })
      .subscribe({
        next: (res) => {},
        error: (err: any) => {
          this.errorService.newError(Source.GeneralService, undefined, err);
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
        .delete<boolean>(
          '/backend/configurations/dnsservers/' + server.ipaddress
        )
        .subscribe({
          next: (res) => {},
          error: (err: any) => {
            this.errorService.newError(Source.GeneralService, undefined, err);
          },
          complete: () => {
            if (i === servers.length) {
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
        this.errorService.newError(Source.GeneralService, undefined, err);
      },
      complete: () => {},
    });
  };

  listSystemDNSServers = () => {
    this.http
      .get<DNSServer[]>('/backend/systeminformation/dnsservers')
      .subscribe({
        next: (res) => {
          this.dataStore.systemDNSServers = res;
          this._systemDNSServers.next(this.dataStore.systemDNSServers.slice());
        },
        error: (err: any) => {
          this.errorService.newError(Source.GeneralService, undefined, err);
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
        this.errorService.newError(Source.GeneralService, undefined, err);
      },
      complete: () => {},
    });
  };

  uploadConfigFile = (config: Configuration, password: string) => {
    const ref = this;

    const subscriptionOTK = this.encryptionService
      .requestOneTimeKey()
      .subscribe({
        next(otk) {
          ref.upload(otk, config, password);
        },
        error(err) {
          ref.errorService.newError(Source.GeneralService, undefined, err);
        },
        complete() {
          subscriptionOTK.unsubscribe();
        },
      });
  };

  upload = (otk: OneTimeKey, config: Configuration, password: string) => {
    const body = JSON.stringify(config);

    const encrypted_password = this.encryptionService.encrypt(
      password,
      this.encryptionService.makeSecret('config', otk.key)
    );
    const headers = new HttpHeaders({
      'X-custom': `${otk.id}`,
      'X-custom2': `${encrypted_password}`,
      'Content-Type': 'application/json',
    });
    this.http
      .post<boolean>('/backend/configuration', body, {
        headers: headers,
      })
      .subscribe({
        next: (res) => {
          this.logger.debug(res);
        },
        error: (err: any) => {
          this.errorService.newError(Source.GeneralService, undefined, err);
        },
        complete: () => {
          setTimeout(this.listDNSServers, 200);
        },
      });
  };

  getConfig = (password: string): Observable<Configuration> => {
    return this.encryptionService.requestOneTimeKey().pipe(
      mergeMap((otk) => {
        const encrypted_password = this.encryptionService.encrypt(
          password,
          this.encryptionService.makeSecret('config', otk.key)
        );
        const headers = new HttpHeaders({
          'X-custom': `${otk.id}`,
          'X-custom2': `${encrypted_password}`,
        });
        const httpOptions = {
          headers: headers,
        };
        return this.http
          .get<Configuration>('/backend/configuration', httpOptions)
          .pipe(
            catchError((err) => {
              this.errorService.newError(Source.GeneralService, undefined, err);
              return throwError(() => err);
            })
          );
      })
    );
  };
}
