import { Component, OnDestroy, OnInit } from '@angular/core';
import { Observable } from 'rxjs';
import { Server } from 'src/app/services/servers/types';
import { ImageCache } from 'src/app/services/cache/image-cache.service';

import { Store } from '@ngrx/store';
import { selectAllServers } from 'src/app/state/server/server.selectors';
import { selectAllPlugins } from 'src/app/state/plugin/plugin.selectors';
import { Plugin } from 'src/app/services/plugins/types';
import { SubscriptionHandler } from 'src/app/shared/subscriptionHandler';

@Component({
  selector: 'app-server-list-wrapper',
  templateUrl: './server-list-wrapper.component.html',
  styleUrls: ['./server-list-wrapper.component.scss'],
})
export class ServerListWrapperComponent implements OnInit, OnDestroy {
  servers$: Observable<Server[]>;
  plugins$: Observable<Plugin[]>;

  private subscriptionHandler = new SubscriptionHandler(this);

  constructor(private store: Store, private imageCache: ImageCache) {
    this.servers$ = this.store.select(selectAllServers);
    this.plugins$ = this.store.select(selectAllPlugins);
  }

  ngOnInit(): void {
    this.subscriptionHandler.subscription = this.plugins$.subscribe((plugins) =>
      this.imageCache.init(plugins)
    );
  }

  ngOnDestroy(): void {
    this.subscriptionHandler.onDestroy();
  }
}
