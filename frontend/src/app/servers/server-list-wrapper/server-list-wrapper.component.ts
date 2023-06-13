import { Component, OnDestroy, OnInit } from '@angular/core';
import { PluginService } from 'src/app/services/plugins/plugin.service';
import { ServerStatusService } from 'src/app/services/servers/server-status.service';
import { ServerService } from 'src/app/services/servers/server.service';
import { Subscription } from 'rxjs';
import { Server } from 'src/app/services/servers/types';
import { ImageCache } from 'src/app/services/cache/image-cache.service';
import { ServerActionService } from 'src/app/services/servers/server-action.service';

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
    this.pluginService.loadPlugins();
    this.serverService.listServers();
    this.statusService.listServerStatus(this.servers);
    this.serverActionService.listActionCheckResults();

    setInterval(() => {
      if (this.servers) {
        this.statusService.listServerStatus(this.servers);
      }
    }, 10000);

    setInterval(() => {
      if (this.servers) {
        this.serverActionService.listActionCheckResults();
      }
    }, 10000);
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
