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
} from './notification.actions';
import { NotificationService } from 'src/app/services/notifications/notifications.service';
import { ErrorService, Source } from 'src/app/services/errors/error.service';

@Injectable()
export class NotificationEffects {
  private action$ = inject(Actions);
  private notificationService = inject(NotificationService);
  private errorService = inject(ErrorService);

  loadAll$ = createEffect(() => {
    return this.action$.pipe(
      ofType(loadAll),
      switchMap(() => this.notificationService.listNotifications()),
      map((notifications) => loadAllSuccess({ notifications: notifications })),
      catchError((e) => of(loadAllFailure({ error: e })))
    );
  });

  loadAllSuccess$ = createEffect(() => {
    return this.action$.pipe(
      ofType(loadAllSuccess),
      map((action) => addMany({ notifications: action.notifications }))
    );
  });

  loadAllFailure$ = createEffect(
    () => {
      return this.action$.pipe(
        ofType(loadAllFailure),
        tap((err) =>
          this.errorService.newError(Source.NotificationService, undefined, err)
        )
      );
    },
    { dispatch: false }
  );
}
