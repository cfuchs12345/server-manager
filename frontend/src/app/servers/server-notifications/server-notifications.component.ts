import { Component, OnInit, Input, OnDestroy, OnChanges } from '@angular/core';
import { MatDialog } from '@angular/material/dialog';
import { Server } from 'src/app/services/servers/types';
import { NotificationService } from 'src/app/services/notifications/notifications.service';
import { Notification } from 'src/app/services/notifications/types';
import { Subscription, map } from 'rxjs';

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
    private notificationService: NotificationService,
    private dialog: MatDialog
  ) {}

  ngOnInit(): void {
    this.subscription = this.notificationService.notifications.pipe(
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
