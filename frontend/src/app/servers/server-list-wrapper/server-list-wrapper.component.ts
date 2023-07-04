import { Component, OnDestroy, OnInit } from '@angular/core';
import { PluginService } from 'src/app/services/plugins/plugin.service';
import { ServerStatusService } from 'src/app/services/servers/server-status.service';
import { ServerService } from 'src/app/services/servers/server.service';
import { Subscription, Observable} from 'rxjs';
import { Server } from 'src/app/services/servers/types';
import { ImageCache } from 'src/app/services/cache/image-cache.service';
import { ServerActionService } from 'src/app/services/servers/server-action.service';
import { NotificationService } from 'src/app/services/notifications/notifications.service';

import { Store } from '@ngrx/store';
import { selectAllServers } from 'src/app/state/server/server.selectors';
import { selectAllPlugins } from 'src/app/state/plugin/plugin.selectors';
import { Plugin } from 'src/app/services/plugins/types';

@Component({
  selector: 'app-server-list-wrapper',
  templateUrl: './server-list-wrapper.component.html',
  styleUrls: ['./server-list-wrapper.component.scss'],
})
export class ServerListWrapperComponent implements OnInit, OnDestroy {
  private serverSubscription: Subscription | undefined = undefined;
  private pluginSubscription: Subscription | undefined = undefined;

  servers$: Observable<Server[]>;
  plugins$: Observable<Plugin[]>;

  constructor(
    private store: Store,
    private serverService: ServerService,
    private pluginService: PluginService,
    private statusService: ServerStatusService,
    private serverActionService: ServerActionService,
    private notificationService: NotificationService,
    private imageCache: ImageCache
  ) {
    this.servers$ = this.store.select(selectAllServers);
    this.plugins$ = this.store
      .select(selectAllPlugins);

    this.plugins$.subscribe((plugins) => this.imageCache.init(plugins));
  }

  ngOnInit(): void {
    /*
    setTimeout(this.pluginService.loadPlugins, 0);
    setTimeout(this.serverService.listServers, 0);
    setTimeout(this.statusService.listAllServerStatus, 0);
    setTimeout(this.serverActionService.listActionCheckResults, 0);
    setTimeout(this.notificationService.listNotifications, 0);*/
  }

  ngOnDestroy(): void {
    if (this.serverSubscription) {
      this.serverSubscription.unsubscribe();
    }
    if (this.pluginSubscription) {
      this.pluginSubscription.unsubscribe();
    }
  }
}
