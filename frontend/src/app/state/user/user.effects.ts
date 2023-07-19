// hydration.effects.ts
import { Injectable, inject } from '@angular/core';
import { Actions, createEffect, ofType } from '@ngrx/effects';
import { of } from 'rxjs';
import { map, switchMap, tap, catchError } from 'rxjs/operators';
import {
  addMany,
  loadAll,
  loadAllFailure,
  loadAllSuccess
} from './user.actions';
import { UserService } from 'src/app/services/users/users.service';
import { ErrorService, Source } from 'src/app/services/errors/error.service';

@Injectable()
export class UserEffects {
  private action$ = inject(Actions);
  private userService = inject(UserService);
  private errorService = inject(ErrorService);

  loadAll$ = createEffect(() => {
    return this.action$.pipe(
      ofType(loadAll),
      switchMap(() => this.userService.listUsers()),
      map((users) => loadAllSuccess({ users: users })),
      catchError((e) => of(loadAllFailure({ error: e })))
    );
  });


  loadAllSuccess$ = createEffect(() => {
    return this.action$.pipe(
      ofType(loadAllSuccess),
      map((action) => addMany({ users: action.users }))
    );
  });


  loadAllFailure$ = createEffect(() => {
    return this.action$.pipe(
      ofType(loadAllFailure),
      tap( (err) => this.errorService.newError(Source.UserService, undefined, err)),
    );
  }, { dispatch: false });
}
