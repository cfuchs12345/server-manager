// hydration.effects.ts
import { Injectable } from '@angular/core';
import { Actions, createEffect, ofType } from '@ngrx/effects';
import { of } from 'rxjs';
import { map, tap, mergeMap } from 'rxjs/operators';
import * as GlobalActions from './global.actions';
import { ServerService } from '../services/servers/server.service';
import { PluginService } from '../services/plugins/plugin.service';
import { ServerStatusService } from '../services/servers/server-status.service';
import { ServerActionService } from '../services/servers/server-action.service';
import { NotificationService } from '../services/notifications/notifications.service';
import { removeOne, upsertOne } from './usertoken/usertoken.actions';
import { resetSavedState } from './index';
import { UserService } from '../services/users/users.service';

@Injectable()
export class GlobalEffects {
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
      tap(() => {
        this.pluginService.loadPlugins();
        this.serverService.listServers();
        this.statusService.listAllServerStatus();
        this.serverActionService.listActionCheckResults();
        this.notificationService.listNotifications();
        this.userService.listUsers();
      }),
      map(() => GlobalActions.loadDone())
    );
  });

  logout$ = createEffect(() => {
    return this.action$.pipe(
      ofType(GlobalActions.logout),
      tap(() => resetSavedState()),
      map(() => removeOne())
    );
  });

  constructor(
    private action$: Actions,
    private userService: UserService,
    private serverService: ServerService,
    private pluginService: PluginService,
    private statusService: ServerStatusService,
    private serverActionService: ServerActionService,
    private notificationService: NotificationService
  ) {}
}
