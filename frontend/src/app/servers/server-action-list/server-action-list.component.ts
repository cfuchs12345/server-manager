import {
  Component,
  OnInit,
  Input,
  OnDestroy
} from '@angular/core';
import { Subscription, filter } from 'rxjs';
import { GUIAction } from 'src/app/services/general/types';
import { ImageCache } from 'src/app/services/cache/image-cache.service';
import {
  Server,
  Status,
} from 'src/app/services/servers/types';
import { PluginService } from 'src/app/services/plugins/plugin.service';
import { Plugin } from 'src/app/services/plugins/types';
import { Store } from '@ngrx/store';
import { selectStatusByIpAddress } from 'src/app/state/selectors/status.selectors';

@Component({
  selector: 'app-server-action-list',
  templateUrl: './server-action-list.component.html',
  styleUrls: ['./server-action-list.component.scss'],
})
export class ServerActionListComponent implements OnInit, OnDestroy {
  @Input() server: Server | undefined = undefined;

  guiActions: GUIAction[] = [];
  status: Status | undefined = undefined;
  private plugins: Plugin[] | undefined = undefined;

  private serverStatusSubscription: Subscription | undefined = undefined;
  private pluginSubscription : Subscription | undefined = undefined;
  constructor(
    private imageCache: ImageCache,
      private store: Store,
        private pluginService: PluginService
  ) {}

  ngOnInit(): void {
    if( this.server ) {
      this.serverStatusSubscription = this.store.select(selectStatusByIpAddress(this.server.ipaddress)).subscribe((status) => {
        this.status = status;
      });
    }


    this.pluginSubscription = this.pluginService.plugins.pipe(
      filter( plugins => this.filter(plugins))
    ).subscribe( plugins => this.plugins = plugins);


    this.getActionsForServer();
  }

  ngOnChanges(): void {
    this.getActionsForServer();
  }

  ngOnDestroy(): void {
    if (this.serverStatusSubscription) {
      this.serverStatusSubscription.unsubscribe();
    }
    if( this.pluginSubscription ) {
      this.pluginSubscription.unsubscribe();
    }
  }

  private getActionsForServer = () => {
    if (!this.server || !this.server.features || !this.plugins) {
      return;
    }

    this.guiActions.splice(0, this.guiActions.length);

    this.server.features.forEach((feature) => {
      const plugin = this.plugins?.find( p => p.id === feature.id);

      if (plugin) {
        for (const actionDef of plugin.actions) {

          if( actionDef.show_on_main === false) {
            continue;
          }

          const image = this.imageCache.getImageFeatureAction(
            feature.id,
            actionDef.id
          );

          this.guiActions.push(
            new GUIAction(
              feature,
              actionDef,
              image,
              actionDef.needs_confirmation
            )
          );
        }
      }
    });
  };

  private filter(plugins: Plugin[]) : boolean {
    return plugins.find( p => this.isPluginRequired(p)) !== undefined;
  }
  private isPluginRequired(plugin: Plugin): boolean {
    return this.server !== undefined && this.server.features.find( f => f.id === plugin.id) !== undefined;
  }
}
