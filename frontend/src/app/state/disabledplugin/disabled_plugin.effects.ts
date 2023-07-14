// hydration.effects.ts
import { Injectable } from '@angular/core';
import { Actions, createEffect, ofType } from '@ngrx/effects';
import { of } from 'rxjs';
import { map, tap, catchError, exhaustMap } from 'rxjs/operators';
import {
  addMany,
  disablePlugins,
  loadAll,
  loadAllFailure,
  loadAllSuccess
} from './disabled_plugin.actions';
import { PluginService } from 'src/app/services/plugins/plugin.service';
import { ErrorService, Source } from 'src/app/services/errors/error.service';

@Injectable()
export class DisabledPluginEffects {
  loadAll$ = createEffect(() => {
    return this.action$.pipe(
      ofType(loadAll),
      exhaustMap(() => this.pluginService.loadDisabledPlugins()),
      map((disabled_plugins) => loadAllSuccess({ disabled_plugins: disabled_plugins })),
      catchError((e) => of(loadAllFailure({ error: e })))
    );
  });


  loadAllSuccess$ = createEffect(() => {
    return this.action$.pipe(
      ofType(loadAllSuccess),
      map((action) => addMany({ disabled_plugins: action.disabled_plugins }))
    );
  });


  loadAllFailure$ = createEffect(() => {
    return this.action$.pipe(
      ofType(loadAllFailure),
      tap( (err) => this.errorService.newError(Source.PluginService, undefined, err)),
    );
  }, { dispatch: false });

  disablePlugins$ = createEffect(() => {
    return this.action$.pipe(
      ofType(disablePlugins),
      tap( (action) => this.pluginService.disablePlugins(action.plugins))
    );
  }, { dispatch: false });


  constructor(private action$: Actions, private pluginService: PluginService, private errorService: ErrorService) {}
}
