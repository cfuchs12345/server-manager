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
  constructor(
    private http: HttpClient,
    private errorService: ErrorService,
    private encryptionService: EncryptionService,
    private logger: NGXLogger
  ) {}

  saveDNSServer = (server: DNSServer) => {
    const body = JSON.stringify(server);

    const subscriptipn =  this.http
      .post<boolean>('/backend/configurations/dnsservers', body, {
        headers: defaultHeadersForJSON(),
      })
      .subscribe({
        next: (res) => {},
        error: (err: any) => {
          this.errorService.newError(Source.GeneralService, undefined, err);
        },
        complete: () => {
          subscriptipn.unsubscribe();
        },
      });
  };

  deleteDNSServers = (servers: DNSServer[]) => {
    for (let i = 0; i < servers.length; i++) {
      const server = servers[i];

      const subscriptipn = this.http
        .delete<boolean>(
          '/backend/configurations/dnsservers/' + server.ipaddress
        )
        .subscribe({
          next: (res) => {},
          error: (err: any) => {
            this.errorService.newError(Source.GeneralService, undefined, err);
          },
          complete: () => {
            subscriptipn.unsubscribe();
          },
        });
    }
  };

  listDNSServers = (): Observable<DNSServer[]> => {
    return this.http.get<DNSServer[]>('/backend/configurations/dnsservers').pipe(
      catchError((err) => {
        this.errorService.newError(Source.GeneralService, undefined, err);
        return throwError(() => err);
      })
    );
  };

  listSystemDNSServers = (): Observable<DNSServer[]> => {
    return this.http
      .get<DNSServer[]>('/backend/systeminformation/dnsservers')
      .pipe(

        catchError((err) => {
          this.errorService.newError(Source.GeneralService, undefined, err);
          return throwError(() => err);
        })
      );
  };

  getSystemInformation = (): Observable<SystemInformation> => {
    return this.http.get<SystemInformation>('/backend/system/information').pipe(
      catchError((err) => {
        this.errorService.newError(Source.GeneralService, undefined, err);
        return throwError(() => err);
      })
    );
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
    const subscription = this.http
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
          subscription.unsubscribe();
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
