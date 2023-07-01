import {
  Component,
  OnInit,
  Input,
  ChangeDetectorRef,
  OnDestroy
} from '@angular/core';
import { Subscription, Observable, filter } from 'rxjs';
import { GUIAction } from 'src/app/services/general/types';
import { ImageCache } from 'src/app/services/cache/image-cache.service';
import {
  ConditionCheckResult,
  Server,
  Status,
} from 'src/app/services/servers/types';
import { Plugin } from 'src/app/services/plugins/types';
import { Store } from '@ngrx/store';
import { selectStatusByIpAddress } from 'src/app/state/selectors/status.selectors';
import { selectAllPlugins } from 'src/app/state/selectors/plugin.selectors';
import { selectConditionCheckResultByIpAddress } from 'src/app/state/selectors/conditioncheckresult.selectors';
import { NGXLogger } from 'ngx-logger';

@Component({
  selector: 'app-server-action-list',
  templateUrl: './server-action-list.component.html',
  styleUrls: ['./server-action-list.component.scss'],
})
export class ServerActionListComponent implements OnInit, OnDestroy {
  @Input() server: Server | undefined = undefined;
  conditionCheckResult: ConditionCheckResult | undefined = undefined;

  guiActions: GUIAction[] = [];
  status: Status | undefined = undefined;
  private plugins: Plugin[] | undefined = undefined;

  private serverStatusSubscription: Subscription | undefined = undefined;
  private pluginSubscription: Subscription | undefined = undefined;
  private serverActionCheckSubscription: Subscription | undefined = undefined;

  private conditions$:
    | Observable<ConditionCheckResult | undefined>
    | undefined = undefined;
  private status$: Observable<Status | undefined> | undefined = undefined;
  private plugins$: Observable<Plugin[]>;

  constructor(
    private store: Store,
    private imageCache: ImageCache,
    private logger: NGXLogger,
    private cdr: ChangeDetectorRef
  ) {
    this.plugins$ = this.store.select(selectAllPlugins);
  }

  ngOnInit(): void {
    if (this.server) {
      this.conditions$ = this.store.select(
        selectConditionCheckResultByIpAddress(this.server.ipaddress)
      );

      this.serverActionCheckSubscription = this.conditions$.subscribe(
        (checkResult) => {
          if (checkResult && this.isCheckResultRequired(checkResult)) {
            this.conditionCheckResult = checkResult;

            this.getActionsForServer();
          }
        }
      );
    }

    if (this.server) {
      this.status$ = this.store.select(
        selectStatusByIpAddress(this.server.ipaddress)
      );

      this.serverStatusSubscription = this.status$.subscribe((status) => {
        this.status = status;

        this.getActionsForServer();
      });
    }

    this.pluginSubscription = this.plugins$
      .pipe(filter((plugins) => this.filter(plugins)))
      .subscribe((plugins) => {
        this.plugins = plugins;

        this.getActionsForServer();
      });
  }

  ngOnDestroy(): void {
    if (this.serverStatusSubscription) {
      this.serverStatusSubscription.unsubscribe();
    }
    if (this.pluginSubscription) {
      this.pluginSubscription.unsubscribe();
    }
    if (this.serverActionCheckSubscription) {
      this.serverActionCheckSubscription.unsubscribe();
    }
  }

  private getActionsForServer = () => {
    if (!this.server || !this.server.features || !this.plugins) {
      return;
    }
    this.guiActions.splice(0, this.guiActions.length);

    this.server.features.forEach((feature) => {
      const plugin = this.plugins?.find((p) => p.id === feature.id);

      if (plugin) {
        for (const actionDef of plugin.actions) {
          if (actionDef.show_on_main === false) {
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

  private filter(plugins: Plugin[]): boolean {
    return plugins.find((p) => this.isPluginRequired(p)) !== undefined;
  }
  private isPluginRequired(plugin: Plugin): boolean {
    return (
      this.server !== undefined &&
      this.server.features.find((f) => f.id === plugin.id) !== undefined
    );
  }

  private isCheckResultRequired(
    result: ConditionCheckResult | undefined
  ): boolean {
    return (
      this.server !== undefined &&
      result !== undefined &&
      this.server.ipaddress === result.ipaddress
    );
  }
}
