import { Component, OnDestroy, OnInit } from '@angular/core';
import { PluginService } from 'src/app/services/plugins/plugin.service';
import { ServerStatusService } from 'src/app/services/servers/server-status.service';
import { ServerService } from 'src/app/services/servers/server.service';
import { Subscription } from 'rxjs';
import { Server } from 'src/app/services/servers/types';
import { ImageCache } from 'src/app/services/cache/image-cache.service';
import { ServerActionService } from 'src/app/services/servers/server-action.service';
import { NotificationService } from 'src/app/services/notifications/notifications.service';
import { EventService } from 'src/app/services/events/event.service';

@Component({
  selector: 'app-server-list-wrapper',
  templateUrl: './server-list-wrapper.component.html',
  styleUrls: ['./server-list-wrapper.component.scss'],
})
export class ServerListWrapperComponent implements OnInit, OnDestroy {
  private serverSubscription: Subscription | undefined = undefined;
  private pluginSubscription: Subscription | undefined = undefined;

  servers: Server[] = [];

  constructor(
    private serverService: ServerService,
    private pluginService: PluginService,
    private statusService: ServerStatusService,
    private serverActionService: ServerActionService,
    private notificationService: NotificationService,
    private eventService: EventService,
    private imageCache: ImageCache
  ) {}

  ngOnInit(): void {
    this.serverSubscription = this.serverService.servers.subscribe(
      (servers) => {
        this.servers = servers;
      }
    );

    this.pluginSubscription = this.pluginService.plugins.subscribe(
      (plugins) => {
        this.imageCache.init(plugins);
      }
    );
    setTimeout(this.pluginService.loadPlugins, 0);
    setTimeout(this.serverService.listServers,0);
    setTimeout( () => {this.statusService.listAllServerStatus()}, 0);
    setTimeout(this.serverActionService.listActionCheckResults, 0);
    setTimeout(this.notificationService.listNotifications, 0);



    setInterval(() => {
      if (this.servers) {
        this.serverActionService.listActionCheckResults();
      }
    }, 10000);

    setInterval(() => {
      if (this.servers) {
        this.notificationService.listNotifications();
      }
    }, 30000);
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
