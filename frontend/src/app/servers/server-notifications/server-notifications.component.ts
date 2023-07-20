import { Component, Input, OnChanges, SimpleChanges, inject } from '@angular/core';
import { Server } from 'src/app/services/servers/types';
import { Notifications } from 'src/app/services/notifications/types';
import { Observable } from 'rxjs';
import { Store } from '@ngrx/store';
import { selectNotificationsByIpAddress } from 'src/app/state/notification/notification.selectors';
import { MatTooltipModule } from '@angular/material/tooltip';
import { NgIf, NgFor, AsyncPipe } from '@angular/common';
@Component({
    selector: 'app-server-notifications',
    templateUrl: './server-notifications.component.html',
    styleUrls: ['./server-notifications.component.scss'],
    standalone: true,
    imports: [
        NgIf,
        NgFor,
        MatTooltipModule,
        AsyncPipe,
    ],
})
export class ServerNotificationComponent implements OnChanges {
  private store = inject(Store);

  @Input() server: Server | undefined = undefined;

  notifications$: Observable<Notifications | undefined> | undefined = undefined;

  ngOnChanges(changes: SimpleChanges): void {
    for (const propName in changes) {
      if (Object.hasOwn(changes, propName)) {
        switch (propName) {
          case 'server':
            if (this.server) {
              this.notifications$ = this.store.select(
                selectNotificationsByIpAddress(this.server.ipaddress)
              );
            }
        }
      }
    }
  }
}
