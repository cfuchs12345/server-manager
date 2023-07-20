import { Component, OnDestroy, OnInit, inject } from '@angular/core';
import { Observable } from 'rxjs';
import { Server } from 'src/app/services/servers/types';
import { ImageCache } from 'src/app/services/cache/image-cache.service';

import { Store } from '@ngrx/store';
import { selectAllServers } from 'src/app/state/server/server.selectors';
import { selectAllPlugins } from 'src/app/state/plugin/plugin.selectors';
import { Plugin } from 'src/app/services/plugins/types';
import { SubscriptionHandler } from 'src/app/shared/subscriptionHandler';
import { ServerListComponent } from '../server-list/server-list.component';
import { MatCardModule } from '@angular/material/card';

@Component({
    selector: 'app-server-list-wrapper',
    templateUrl: './server-list-wrapper.component.html',
    styleUrls: ['./server-list-wrapper.component.scss'],
    standalone: true,
    imports: [MatCardModule, ServerListComponent],
})
export class ServerListWrapperComponent implements OnInit, OnDestroy {
  private store = inject(Store);
  private imageCache = inject(ImageCache);

  servers$?: Observable<Server[]>;
  plugins$?: Observable<Plugin[]>;

  private subscriptionHandler = new SubscriptionHandler(this);

  ngOnInit(): void {
    this.servers$ = this.store.select(selectAllServers);
    this.plugins$ = this.store.select(selectAllPlugins);

    this.subscriptionHandler.subscription = this.plugins$.subscribe((plugins) =>
      this.imageCache.init(plugins)
    );
  }

  ngOnDestroy(): void {
    this.subscriptionHandler.onDestroy();
  }
}
