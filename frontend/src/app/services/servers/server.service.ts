import { Injectable } from '@angular/core';
import { HttpClient, HttpParams } from '@angular/common/http';
import { Observable, filter, map, of, switchMap, tap } from 'rxjs';
import { defaultHeadersForJSON } from '../common';
import {
  addMany,
  removeOne,
  upsertOne,
} from 'src/app/state/server/server.actions';
import { Server } from './types';
import { ErrorService, Source } from '../errors/error.service';
import { EncryptionService } from '../encryption/encryption.service';

import { EventService } from '../events/event.service';
import { Event } from '../events/types';
import { Store } from '@ngrx/store';
import { selectToken } from 'src/app/state/usertoken/usertoken.selectors';
import { UserToken } from '../users/types';

@Injectable({
  providedIn: 'root',
})
export class ServerService {
  lastLoad: Date | undefined;

  userToken$: Observable<UserToken | undefined>;

  constructor(
    private store: Store,
    private http: HttpClient,
    private eventService: EventService,
    private errorService: ErrorService,
    private encryptionService: EncryptionService,
  ) {
    this.userToken$ = this.store.select(selectToken());

    this.eventService.eventSubject
      .pipe(
        filter((event: Event) => {
          return event.object_type === 'Server';
        })
      )
      .subscribe((event: Event) => {
        if (event.event_type === 'Insert' || event.event_type === 'Update') {
          const subscription = this.getServer(event.key, false).subscribe({
            next: (server) => {
              this.store.dispatch(upsertOne({ server: server }));
            },
            complete: () => {
              subscription.unsubscribe();
            },
          });
        } else if (event.event_type === 'Delete') {
          this.store.dispatch(removeOne({ ipaddress: event.key }));
        }
      });
  }

  deleteServers = (servers: Server[]) => {
    for (const [, server] of servers.entries()) {
      const subscription = this.http
        .delete(`/backend/servers/${server.ipaddress}`, {
          headers: defaultHeadersForJSON(),
        })
        .subscribe({
          error: (err) => {
            this.errorService.newError(
              Source.ServerService,
              server.ipaddress,
              err
            );
          },
          complete: () => {
            subscription.unsubscribe();
          },
        });
    }
  };

  listServers = () => {
    const subscription = this.http
      .get<Server[]>('/backend/servers')
      .pipe(
        tap(() => {
          this.lastLoad = new Date();
        })
      )
      .subscribe({
        next: (servers) => {
          this.store.dispatch(addMany({ servers: servers })); // add many
        },
        error: (err) => {
          this.errorService.newError(Source.ServerService, undefined, err);
        },
        complete: () => {
          subscription.unsubscribe();
        },
      });
  };

  getServer = (ipaddress: string, fullData: boolean): Observable<Server> => {
    const options = {
      params: new HttpParams().set('full_data', fullData ? 'true' : 'false'),
    };

    return this.http
      .get<Server>(`/backend/servers/${ipaddress}`, options)
      .pipe(
        switchMap(
          (server, ): Observable<Server> =>
            fullData ? this.decryptIfNecessary(server) : of(server)
        )
      );
  };

  private decryptIfNecessary = (server: Server): Observable<Server> => {
    return this.userToken$.pipe(
      map((userToken) => {
        if (!userToken || !userToken.token) {
          return server;
        }
        const key = userToken.client_key;

        for (const feature of server.features) {
          if (!feature.credentials.find((credential) => credential.encrypted)) {
            return server;
          }

          feature.credentials.forEach((credential) => {
            if (credential.encrypted && key) {
              credential.encrypted = !credential.encrypted;
              credential.value = this.encryptionService.decrypt(
                credential.value,
                key
              );
            }
          });
        }
        return server;
      })
    );
  };

  saveServers = (servers: Server[]) => {
    for (const [, server] of servers.entries()) {
      const body = JSON.stringify(server);

      const subscription = this.http
        .post('/backend/servers', body, {
          headers: defaultHeadersForJSON(),
        })
        .subscribe({
          next: () => {
            const preLimServerUpdate = Object.assign({}, server);
            preLimServerUpdate.isPreliminary = true;

            this.store.dispatch(upsertOne({ server: preLimServerUpdate }));
          },
          error: (err) => {
            this.errorService.newError(
              Source.ServerService,
              server.ipaddress,
              err
            );
          },
          complete: () => {
            subscription.unsubscribe();
          },
        });
    }
  };

  updateServer = (server: Server) => {
    const body = JSON.stringify(server);

    const subscription = this.http
      .put(`/backend/servers/${server.ipaddress}`, body, {
        headers: defaultHeadersForJSON(),
      })
      .subscribe({
        error: (err) => {
          this.errorService.newError(
            Source.ServerService,
            server.ipaddress,
            err
          );
        },
        complete: () => {
          subscription.unsubscribe();
        },
      });
  };
}
