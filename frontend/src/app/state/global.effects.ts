// hydration.effects.ts
import { Injectable , inject} from '@angular/core';
import { Actions, createEffect, ofType } from '@ngrx/effects';
import { of } from 'rxjs';
import { mergeMap } from 'rxjs/operators';
import * as GlobalActions from './global.actions';
import { upsertOne } from './usertoken/usertoken.actions';
import * as ServerActions from './server/server.actions';
import * as PluginActions from './plugin/plugin.actions';
import * as DisabledPluginActions from './disabledplugin/disabled_plugin.actions';
import * as UserActions from './user/user.actions';
import * as StatusActions from './status/status.actions';
import * as NotificationsActions from './notification/notification.actions';
import * as ConditionCheckResultActions from './conditioncheckresult/conditioncheckresult.actions';

@Injectable()
export class GlobalEffects {
  private action$ = inject(Actions);

  init$ = createEffect(() => {
    return this.action$.pipe(
      ofType(GlobalActions.init),
      mergeMap((action) =>
        of(upsertOne({ usertoken: action.userToken }), GlobalActions.loadAll())
      )
    );
  });

  loadAll$ = createEffect(() => {
    return this.action$.pipe(
      ofType(GlobalActions.loadAll),
      mergeMap(() =>
        of(
          GlobalActions.loadDone(),
          ServerActions.loadAll(),
          PluginActions.loadAll(),
          DisabledPluginActions.loadAll(),
          UserActions.loadAll(),
          StatusActions.loadAll(),
          NotificationsActions.loadAll(),
          ConditionCheckResultActions.loadAll(),
        )
      )
    );
  });
}
