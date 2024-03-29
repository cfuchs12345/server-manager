// hydration.effects.ts
import { Injectable, inject } from '@angular/core';
import { Actions, createEffect, ofType } from '@ngrx/effects';
import { of, tap } from 'rxjs';
import { map, switchMap,  catchError } from 'rxjs/operators';
import {
  executeAction,
  executeActionSuccess,
  executeActionFailure
} from './action.actions';
import { ErrorService, Source } from 'src/app/services/errors/error.service';
import { ServerActionService } from 'src/app/services/servers/server-action.service';

@Injectable()
export class ActionEffects {
  private action$ = inject(Actions);
  private serverActionService= inject( ServerActionService);
  private errorService= inject( ErrorService);

  executeAction$ = createEffect(() => {
    return this.action$.pipe(
      ofType(executeAction),
      switchMap((action) => this.serverActionService.executeAction(action.feature_id, action.action_id, action.ipaddress, action.action_params)),
      map(() => executeActionSuccess()),
      catchError((e) => of(executeActionFailure({ error: e })))
    );
  });

  executeActionFailure$ = createEffect(() => {
    return this.action$.pipe(
      ofType(executeActionFailure),
      tap( (err) => this.errorService.newError(Source.ServerActionService, undefined, err)),
    );
  }, { dispatch: false });
}
