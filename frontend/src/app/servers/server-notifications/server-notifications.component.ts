import { Component, OnInit, Input, OnDestroy, OnChanges } from '@angular/core';
import { Server } from 'src/app/services/servers/types';
import { Notification } from 'src/app/services/notifications/types';
import { Subscription, map } from 'rxjs';
import { Store } from '@ngrx/store';
import { selectAllNotification } from 'src/app/state/selectors/notification.selectors';
@Component({
  selector: 'app-server-notifications',
  templateUrl: './server-notifications.component.html',
  styleUrls: ['./server-notifications.component.scss'],
})
export class ServerNotificationComponent implements OnInit, OnDestroy, OnChanges {
  @Input() server: Server | undefined = undefined;

  notifications: Notification[] | undefined;

  private subscription: Subscription | undefined;

  constructor(
    private store: Store,
  ) {}

  ngOnInit(): void {
    this.subscription = this.store.select(selectAllNotification).pipe(
      map((notifications) => {
        return notifications.filter((n) => n.ipaddress === this.server?.ipaddress);
      })
    ).subscribe((notifications) => {
      this.notifications = notifications;
    });
  }

  ngOnChanges(): void {
  }

  ngOnDestroy(): void {
    if( this.subscription ) {
      this.subscription.unsubscribe();
    }
  }

}
