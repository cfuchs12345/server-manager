// hydration.effects.ts
import { Injectable } from '@angular/core';
import { Actions, createEffect, ofType, OnInitEffects } from '@ngrx/effects';
import { Action, Store } from '@ngrx/store';
import { distinctUntilChanged, map, switchMap, tap } from 'rxjs/operators';
import * as HydrationActions from './hydration.actions';
import { resetSavedState, getSavedState, savedState, State } from '..';

@Injectable()
export class HydrationEffects implements OnInitEffects {
  hydrate$ = createEffect(() => {
    return this.action$.pipe(
      ofType(HydrationActions.hydrate),
      map(() => {
        try {
          return HydrationActions.hydrateSuccess({ state: getSavedState() });
        } catch {
          resetSavedState();
          return HydrationActions.hydrateFailure();
        }
      })
    );
  });

  serialize$ = createEffect(
    () => {
      return this.action$.pipe(
        ofType(
          HydrationActions.hydrateSuccess,
          HydrationActions.hydrateFailure
        ),
        switchMap(() => this.store),
        distinctUntilChanged(),
        tap((state) => savedState(state as State))
      );
    },
    { dispatch: false }
  );

  constructor(
    private action$: Actions,
    private store: Store
  ) {}

  ngrxOnInitEffects(): Action {
    return HydrationActions.hydrate();
  }
}
