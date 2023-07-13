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
} from './conditioncheckresult.actions';
import { ServerActionService } from 'src/app/services/servers/server-action.service';
import { ErrorService, Source } from 'src/app/services/errors/error.service';

@Injectable()
export class ConditionCheckResultEffects {
  loadAll$ = createEffect(() => {
    return this.action$.pipe(
      ofType(loadAll),
      switchMap(() => this.serverActionService.listActionCheckResults()),
      map((results) => loadAllSuccess({ results: results })),
      catchError((e) => of(loadAllFailure({ error: e })))
    );
  });


  loadAllSuccess$ = createEffect(() => {
    return this.action$.pipe(
      ofType(loadAllSuccess),
      map((action) => addMany({ results: action.results }))
    );
  });


  loadAllFailure$ = createEffect(() => {
    return this.action$.pipe(
      ofType(loadAllFailure),
      tap( (err) => this.errorService.newError(Source.PluginService, undefined, err)),
    );
  }, { dispatch: false });


  constructor(private action$: Actions, private serverActionService: ServerActionService, private errorService: ErrorService) {}
}
