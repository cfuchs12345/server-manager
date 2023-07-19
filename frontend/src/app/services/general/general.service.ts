import { Injectable } from '@angular/core';
import { HttpClient, HttpHeaders } from '@angular/common/http';
import { Observable, catchError, throwError, mergeMap, take } from 'rxjs';
import { defaultHeadersForJSON } from '../common';
import { Configuration, DNSServer } from './types';
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

    this.http
      .post<boolean>('/backend/configurations/dnsservers', body, {
        headers: defaultHeadersForJSON(),
      })
      .pipe(take(1))
      .subscribe({
        error: (err) => {
          this.errorService.newError(Source.GeneralService, undefined, err);
        },
      });
  };

  deleteDNSServers = (servers: DNSServer[]) => {
    for (let i = 0; i < servers.length; i++) {
      const server = servers[i];

      this.http
        .delete<boolean>(
          '/backend/configurations/dnsservers/' + server.ipaddress
        )
        .pipe(take(1))
        .subscribe({
          error: (err) => {
            this.errorService.newError(Source.GeneralService, undefined, err);
          },
        });
    }
  };

  listDNSServers = (callback: (dnsservers: DNSServer[]) => void) => {
    this.http
      .get<DNSServer[]>('/backend/configurations/dnsservers')
      .pipe(
        take(1),
        catchError((err) => {
          this.errorService.newError(Source.GeneralService, undefined, err);
          return throwError(() => err);
        })
      )
      .subscribe({
        next: (value) => {
          callback(value);
        },
      });
  };

  listSystemDNSServers = (callback: (dnsservers: DNSServer[]) => void) => {
    this.http
      .get<DNSServer[]>('/backend/systeminformation/dnsservers')
      .pipe(
        take(1),
        catchError((err) => {
          this.errorService.newError(Source.GeneralService, undefined, err);
          return throwError(() => err);
        })
      )
      .subscribe({
        next: (value) => {
          callback(value);
        },
      });
  };

  uploadConfigFile = (config: Configuration, password: string) => {
    this.encryptionService
      .requestOneTimeKey()
      .pipe(take(1))
      .subscribe({
        next: (otk) => {
          this.upload(otk, config, password);
        },
        error: (err) => {
          this.errorService.newError(Source.GeneralService, undefined, err);
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
      .pipe(take(1))
      .subscribe({
        next: (res) => {
          this.logger.trace(res);
        },
        error: (err) => {
          this.errorService.newError(Source.GeneralService, undefined, err);
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
