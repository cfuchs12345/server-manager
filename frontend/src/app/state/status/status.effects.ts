// hydration.effects.ts
import { Injectable } from '@angular/core';
import { Actions, createEffect, ofType } from '@ngrx/effects';
import { of } from 'rxjs';
import { map, switchMap, tap, catchError } from 'rxjs/operators';
import {
  addMany,
  loadAll,
  loadAllFailure,
  loadAllSuccess
} from './status.actions';
import { ServerStatusService } from 'src/app/services/servers/server-status.service';
import { ErrorService, Source } from 'src/app/services/errors/error.service';

@Injectable()
export class StatusEffects {
  loadAll$ = createEffect(() => {
    return this.action$.pipe(
      ofType(loadAll),
      switchMap(() => this.serverStatusService.listAllServerStatus()),
      map((status) => loadAllSuccess({ status: status })),
      catchError((e) => of(loadAllFailure({ error: e })))
    );
  });


  loadAllSuccess$ = createEffect(() => {
    return this.action$.pipe(
      ofType(loadAllSuccess),
      map((action) => addMany({ status: action.status }))
    );
  });


  loadAllFailure$ = createEffect(() => {
    return this.action$.pipe(
      ofType(loadAllFailure),
      tap( (err) => this.errorService.newError(Source.ServerStatusService, undefined, err)),
    );
  }, { dispatch: false });


  constructor(private action$: Actions, private serverStatusService: ServerStatusService, private errorService: ErrorService) {}
}
