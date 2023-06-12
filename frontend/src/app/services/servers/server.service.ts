import { Injectable } from '@angular/core';
import { HttpClient, HttpParams } from '@angular/common/http';
import {
  BehaviorSubject,
  Observable,
  catchError,
  map,
  tap,
  throwError,
} from 'rxjs';
import { defaultHeadersForJSON } from '../common';

import { Server, Feature, ServerFeature } from './types';
import { ErrorService, Source } from '../errors/error.service';
import { EncryptionService } from '../encryption/encryption.service';
import { AuthenticationService } from '../auth/authentication.service';

@Injectable({
  providedIn: 'root',
})
export class ServerService {
  private _servers = new BehaviorSubject<Server[]>([]);
  private dataStore: {
    servers: Server[];
  } = {
    servers: [],
  };

  readonly servers = this._servers.asObservable();

  constructor(
    private http: HttpClient,
    private errorService: ErrorService,
    private encryptionService: EncryptionService,
    private authService: AuthenticationService
  ) {}

  deleteServers = (servers: Server[]) => {
    for (const [i, server] of servers.entries()) {
      this.http
        .delete<any>(`/backend/servers/${server.ipaddress}`, {
          headers: defaultHeadersForJSON(),
        })
        .subscribe({
          next: (res) => {
            const indexToDelete = this.dataStore.servers.findIndex(
              (s) => s.ipaddress === server.ipaddress
            );
            this.dataStore.servers.splice(indexToDelete);
          },
          error: (err: any) => {
            this.errorService.newError(
              Source.ServerService,
              server.ipaddress,
              err
            );
          },
          complete: () => {
            if (servers[servers.length - 1].ipaddress === server.ipaddress) {
              setTimeout(this.publishServers, 500);
            }
          },
        });
    }
  };

  listServers = () => {
    this.http.get<Server[]>('/backend/servers').subscribe({
      next: (servers) => {
        this.dataStore.servers = servers;
      },
      error: (err: any) => {
        this.errorService.newError(Source.ServerService, undefined, err);
      },
      complete: () => {
        setTimeout(this.publishServers, 500);
      },
    });
  };

  getServer = (ipaddress: string, fullData: boolean): Observable<Server> => {
    const options = fullData
      ? {
          params: new HttpParams().set(
            'full_data',
            fullData ? 'true' : 'false'
          ),
        }
      : {};

    return this.http.get<Server>(`/backend/servers/${ipaddress}`, options).pipe(
      catchError((err) => {
        this.errorService.newError(Source.ServerService, ipaddress, err);
        return throwError(() => err);
      }),
      tap((server) => {
        if (fullData) {

          if (server.features) {
            server.features.forEach((feature: Feature) => {
              this.decryptIfNecessary(feature);
            });
          }

          console.log(server);
        }
      }),
      catchError((err) => {
        this.errorService.newError(Source.ServerService, ipaddress, err);
        return throwError(() => err);
      }),
    );
  };

  private decryptIfNecessary = (feature: Feature) => {
    if (!feature.credentials.find((credential) => credential.encrypted)) {
      return;
    }
    const key = this.authService.userToken?.client_key;

    feature.credentials.forEach((credential) => {
      if (credential.encrypted && key) {
        credential.encrypted = !credential.encrypted;
        credential.value = this.encryptionService.decrypt(credential.value, key);
      }
    });
  };


  saveServers = (servers: Server[]) => {
    for (const [i, server] of servers.entries()) {
      const body = JSON.stringify(server);

      this.http
        .post<any>('/backend/servers', body, {
          headers: defaultHeadersForJSON(),
        })
        .subscribe({
          next: (res) => {},
          error: (err: any) => {
            this.errorService.newError(
              Source.ServerService,
              server.ipaddress,
              err
            );
          },
          complete: () => {},
        });
    }
  };

  updateServer = (server: Server) => {
    const body = JSON.stringify(server);

    this.http
      .put<any>(`/backend/servers/${server.ipaddress}`, body, {
        headers: defaultHeadersForJSON(),
      })
      .subscribe({
        next: (res) => {},
        error: (err: any) => {
          this.errorService.newError(
            Source.ServerService,
            server.ipaddress,
            err
          );
        },
        complete: () => {},
      });
  };

  private publishServers = () => {
    this.dataStore.servers.sort(this.compareServers);
    this._servers.next(this.dataStore.servers.slice());
  };

  private compareServers = (a: Server, b: Server): number => {
    const numA = Number(
      a.ipaddress
        .split('.')
        .map((num, idx) => parseInt(num) * Math.pow(2, (3 - idx) * 8))
        .reduce((a, v) => ((a += v), a), 0)
    );
    const numB = Number(
      b.ipaddress
        .split('.')
        .map((num, idx) => parseInt(num) * Math.pow(2, (3 - idx) * 8))
        .reduce((a, v) => ((a += v), a), 0)
    );
    return numA - numB;
  };

  private updateOrAddFeature = (
    foundFeature: Feature[],
    server: Server,
    overwriteParams: boolean
  ): Server => {
    for (var feature of foundFeature) {
      var existing = server.features.find((f) => f.id === feature.id);

      if (!existing) {
        server.features.push(
          new Feature(
            feature.id,
            feature.name,
            feature.params,
            feature.credentials
          )
        );
      } else if (overwriteParams) {
        existing.name = feature.name;
        existing.params = feature.params;
        existing.credentials = feature.credentials;
      }
    }
    return server;
  };

  private removeFeaturesNoLongerInList = (
    featuresToSetForServer: Feature[],
    server: Server
  ): Server => {
    server.features = server.features.filter((existingFeature) =>
      this.isInList(existingFeature, featuresToSetForServer)
    );

    return server;
  };

  private isInList = (
    featureToCheck: Feature,
    featureListToCheck: Feature[]
  ): boolean => {
    return (
      featureListToCheck.find((feature) => feature.id == featureToCheck.id) !==
      undefined
    );
  };
}
