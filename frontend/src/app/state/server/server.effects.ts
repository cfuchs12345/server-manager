// hydration.effects.ts
import { Injectable } from '@angular/core';
import { Actions, createEffect, ofType } from '@ngrx/effects';
import { of } from 'rxjs';
import { map, switchMap, tap, catchError } from 'rxjs/operators';
import {
  addMany,
  loadAll,
  loadAllFailure,
  loadAllSuccess,
  loadSpecific,
  loadSpecificFailure,
  loadSpecificSuccess,
  upsertOne,
} from './server.actions';
import { ServerService } from 'src/app/services/servers/server.service';
import { ErrorService, Source } from 'src/app/services/errors/error.service';

@Injectable()
export class ServerEffects {
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

  loadAllFailure$ = createEffect(() => {
    return this.action$.pipe(
      ofType(loadAllFailure),
      tap( (err) => this.errorService.newError(Source.ServerService, undefined, err)),
    );
  }, { dispatch: false });

  loadSpecificFailure$ = createEffect(() => {
    return this.action$.pipe(
      ofType(loadSpecificFailure),
      tap( (err) => this.errorService.newError(Source.ServerService, undefined, err)),
    );
  }, { dispatch: false });

  constructor(private action$: Actions, private serverService: ServerService, private errorService: ErrorService) {}
}
