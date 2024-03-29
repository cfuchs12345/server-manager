// hydration.effects.ts
import { Injectable, inject } from '@angular/core';
import { Actions, createEffect, ofType } from '@ngrx/effects';
import { of } from 'rxjs';
import { map, switchMap, tap, catchError } from 'rxjs/operators';
import {
  addMany,
  loadAll,
  loadAllFailure,
  loadAllSuccess,
} from './plugin.actions';
import { PluginService } from 'src/app/services/plugins/plugin.service';
import { ErrorService, Source } from 'src/app/services/errors/error.service';

@Injectable()
export class PluginEffects {
  private action$ = inject(Actions);
  private pluginService = inject(PluginService);
  private errorService = inject(ErrorService);

  loadAll$ = createEffect(() => {
    return this.action$.pipe(
      ofType(loadAll),
      switchMap(() => this.pluginService.loadPlugins()),
      map((plugins) => loadAllSuccess({ plugins: plugins })),
      catchError((e) => of(loadAllFailure({ error: e })))
    );
  });

  loadAllSuccess$ = createEffect(() => {
    return this.action$.pipe(
      ofType(loadAllSuccess),
      map((action) => addMany({ plugins: action.plugins }))
    );
  });

  loadAllFailure$ = createEffect(
    () => {
      return this.action$.pipe(
        ofType(loadAllFailure),
        tap((err) =>
          this.errorService.newError(Source.PluginService, undefined, err)
        )
      );
    },
    { dispatch: false }
  );
}
