// hydration.effects.ts
import { Injectable, inject } from '@angular/core';
import { Actions, createEffect, ofType } from '@ngrx/effects';
import { Observable, of } from 'rxjs';
import { map, switchMap, tap, catchError, mergeMap } from 'rxjs/operators';
import {
  addMany,
  loadAll,
  loadAllFailure,
  loadAllSuccess,
  loadSpecific,
  loadSpecificFailure,
  loadSpecificSuccess,
  removeServerFeature,
  removeServerFeatureSuccess,
  addServerFeatures as addServerFeatures,
  upsertOne,
  addServerFeature,
  removeServers,
  saveServers,
  saveServer,
} from './server.actions';
import { ServerService } from 'src/app/services/servers/server.service';
import { ErrorService, Source } from 'src/app/services/errors/error.service';
import { Server, ServerFeature } from 'src/app/services/servers/types';

@Injectable()
export class ServerEffects {
  private action$ = inject(Actions);
  private serverService = inject(ServerService);
  private errorService = inject(ErrorService);

  loadAll$ = createEffect(() => {
    return this.action$.pipe(
      ofType(loadAll),
      switchMap(() => this.serverService.listServers()),
      map((servers) => loadAllSuccess({ servers: servers })),
      catchError((e) => of(loadAllFailure({ error: e })))
    );
  });

  loadSpecific$ = createEffect(() => {
    return this.action$.pipe(
      ofType(loadSpecific),
      switchMap((action) =>
        this.serverService.getServer(action.ipaddress, action.fullData)
      ),
      map((server) => loadSpecificSuccess({ server: server })),
      catchError((e) => of(loadSpecificFailure({ error: e })))
    );
  });

  loadAllSuccess$ = createEffect(() => {
    return this.action$.pipe(
      ofType(loadAllSuccess),
      map((action) => addMany({ servers: action.servers }))
    );
  });

  loadSpecificSuccess$ = createEffect(() => {
    return this.action$.pipe(
      ofType(loadSpecificSuccess),
      map((action) => upsertOne({ server: action.server }))
    );
  });

  loadAllFailure$ = createEffect(
    () => {
      return this.action$.pipe(
        ofType(loadAllFailure),
        tap((err) =>
          this.errorService.newError(Source.ServerService, undefined, err)
        )
      );
    },
    { dispatch: false }
  );

  loadSpecificFailure$ = createEffect(
    () => {
      return this.action$.pipe(
        ofType(loadSpecificFailure),
        tap((err) =>
          this.errorService.newError(Source.ServerService, undefined, err)
        )
      );
    },
    { dispatch: false }
  );

  addServerFeautures$ = createEffect(
    () => {
      return this.action$.pipe(
        ofType(addServerFeatures),
        mergeMap((action) => {
          const observables: Observable<Server>[] = [];
          action.serverFeatures.forEach((foundServerFeature) => {
            observables.push(this.saveSingleFeature(foundServerFeature));
          });
          return of(observables);
        })
      );
    },
    { dispatch: false }
  );

  addServerFeature$ = createEffect(
    () => {
      return this.action$.pipe(
        ofType(addServerFeature),
        mergeMap((action) => this.saveSingleFeature(action.serverFeature))
      );
    },
    { dispatch: false }
  );

  removeServerFeature$ = createEffect(() => {
    return this.action$.pipe(
      ofType(removeServerFeature),
      tap((action) => {
        // cannot get it from store here, since we need the full data (features, credentials, params and so on)
        this.serverService
          .getServer(action.serverFeature.ipaddress, true)
          .subscribe({
            next: (server) => {
              const filteredFeatures = server.features.filter(
                (feature) =>
                  action.serverFeature.features.find(
                    (f) => f.id === feature.id
                  ) === undefined
              );
              server.features = filteredFeatures;

              this.serverService.updateServer(server);
            },
          });
      }),
      map(() => removeServerFeatureSuccess())
    );
  });

  saveServer$ = createEffect(() => {
    return this.action$.pipe(
      ofType(saveServer),
      tap((action) => {
        this.serverService.saveServers([action.server]);
      })
    );
  },  { dispatch: false });

  saveServers$ = createEffect(() => {
    return this.action$.pipe(
      ofType(saveServers),
      tap((action) => {
        this.serverService.saveServers(action.servers);
      })
    );
  },  { dispatch: false });

  removeServers$ = createEffect(() => {
    return this.action$.pipe(
      ofType(removeServers),
      tap((action) => {
        this.serverService.deleteServers(action.servers);
      })
    );
  }, { dispatch: false });


  private saveSingleFeature = (
    foundServerFeature: ServerFeature
  ): Observable<Server> => {
    return this.serverService
      .getServer(foundServerFeature.ipaddress, true)
      .pipe(
        tap((server) => {
          let updated = false;

          foundServerFeature.features.forEach((found) => {
            const already_set = server.features.find(
              (server_feature) => server_feature.id === found.id
            );

            if (!already_set) {
              updated = true;
              server.features.push(found);
            }
          }); // we just add new features found, removal is only done manually

          if (updated) {
            this.serverService.updateServer(server);
          }
        })
      );
  };
}
