import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import {
  addMany,
  removeOne,
  upsertOne,
} from 'src/app/state/actions/notification.action';
import { Notification } from './types';
import { Observable, filter, tap } from 'rxjs';
import { ErrorService, Source } from '../errors/error.service';
import { Store } from '@ngrx/store';
import { EventService } from '../events/event.service';
import { Event } from '../events/types';

@Injectable({
  providedIn: 'root',
})
export class NotificationService {
  constructor(
    private store: Store,
    private http: HttpClient,
    private errorService: ErrorService,
    private eventService: EventService
  ) {
    this.eventService.eventSubject
      .pipe(
        filter((event: Event) => {
          return event.object_type === 'Notification';
        })
      )
      .subscribe((event: Event) => {
        if (event.event_type === 'Insert' || event.event_type === 'Update') {
          const notification: Notification = JSON.parse(event.value);

          this.store.dispatch(upsertOne({notification: notification}));
        } else if (event.event_type === 'Delete') {
          this.store.dispatch(removeOne({ ipaddress: event.key }));
        }
      });
  }

  listNotifications = () => {
    const subscription = this.http
      .get<Notification[]>('/backend/notifications')
      .subscribe({
        next: (notifications) => {
          this.store.dispatch(addMany({ notifications: notifications }));
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
        complete: () => {
          subscription.unsubscribe();
        },
      });
  };
}
