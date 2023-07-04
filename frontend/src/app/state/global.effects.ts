// hydration.effects.ts
import { Injectable } from '@angular/core';
import { Actions, createEffect, ofType } from '@ngrx/effects';
import { map, tap } from 'rxjs/operators';
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
  loadAll$ = createEffect(() => {
    return this.action$.pipe(
      ofType(GlobalActions.init),
      tap(() => {
        this.pluginService.loadPlugins();
        this.serverService.listServers();
        this.statusService.listAllServerStatus();
        this.serverActionService.listActionCheckResults();
        this.notificationService.listNotifications();
        this.userService.listUsers();
      }),
      map((action) => upsertOne({ usertoken: action.userToken }))
    );
  });

  logout$ = createEffect(() => {
    return this.action$.pipe(
      ofType(GlobalActions.logout),
      tap(() => {
        resetSavedState();
      }),
      map((action) => removeOne({ user_id: action.userToken.user_id }))
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
