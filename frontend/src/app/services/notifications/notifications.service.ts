import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import {
  removeOne,
  upsertOne,
} from 'src/app/state/notification/notification.actions';
import { Notifications } from './types';
import { filter, Observable } from 'rxjs';
import { ErrorService } from '../errors/error.service';
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
    this.eventService.eventSubject$
      .pipe(
        filter((eventAndObject: [Event, Notifications]) => {
          const event = eventAndObject[0];

          return event.object_type === 'Notification';
        })
      )
      .subscribe((eventAndObject: [Event, Notifications]) => {
        const event = eventAndObject[0];

        if (event.event_type === 'Insert' || event.event_type === 'Update') {
          const notifications: Notifications = JSON.parse(event.value);

          this.store.dispatch(upsertOne({ notifications: notifications }));
        } else if (event.event_type === 'Delete') {
          this.store.dispatch(removeOne({ ipaddress: event.key }));
        }
      });
  }

  listNotifications = (): Observable<Notifications[]> => {
    return this.http.get<Notifications[]>('/backend/notifications');
  };
}
