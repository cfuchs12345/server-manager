import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { BehaviorSubject } from 'rxjs';
import { defaultHeadersForJSON } from '../common';

import { Notification } from './types';
import { ErrorService, Source } from '../errors/error.service';

@Injectable({
  providedIn: 'root',
})
export class NotificationService {
  private _notifications = new BehaviorSubject<Notification[]>([]);
  readonly notifications = this._notifications.asObservable();

  constructor(private http: HttpClient, private errorService: ErrorService) {}

  listNotifications = () => {
    this.http
      .get<Notification[]>('/backend/notifications')
      .subscribe({
        next: (notifications) => {
          this.publishNotifications(notifications);
        },
        error: (err: any) => {
          if (err) {
            this.errorService.newError(
              Source.NotificationService,
              undefined,
              err
            );
          }
        },
        complete: () => {},
      });
  };

  private publishNotifications = (notifications: Notification[]) => {
    this._notifications.next(notifications);
  };
}
