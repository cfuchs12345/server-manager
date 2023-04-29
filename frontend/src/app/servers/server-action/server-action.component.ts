import {
  Component,
  OnInit,
  Input,
  OnDestroy,
  OnChanges
} from '@angular/core';
import { MatDialog } from '@angular/material/dialog';
import { Subscription, filter, map, tap } from 'rxjs';
import { GUIAction } from 'src/app/services/general/types';
import {
  ConditionCheckResult,
  Server,
  Status,
} from 'src/app/services/servers/types';
import { ConfirmDialogComponent } from 'src/app/ui/confirm-dialog/confirm-dialog.component';
import { ServerStatusService } from 'src/app/services/servers/server-status.service';
import { ServerActionService } from 'src/app/services/servers/server-action.service';
import { PluginService } from 'src/app/services/plugins/plugin.service';
import { Plugin } from 'src/app/services/plugins/types';

@Component({
  selector: 'app-server-action',
  templateUrl: './server-action.component.html',
  styleUrls: ['./server-action.component.scss'],
})
export class ServerActionComponent implements OnInit, OnDestroy, OnChanges {
  @Input() server: Server | undefined = undefined;
  @Input() guiAction: GUIAction | undefined = undefined;
  @Input() status: Status | undefined = undefined;

  plugins: Plugin[] | undefined = undefined;

  allDependenciesMet: boolean = false;

  private conditionCheckResults: ConditionCheckResult[] | undefined = undefined;

  private serverStatusSubscription: Subscription | undefined = undefined;
  private serverActionCheckSubscription: Subscription | undefined = undefined;
  private pluginSubscription : Subscription | undefined = undefined;

  constructor(
    private pluginService: PluginService,
    private serverActionService: ServerActionService,
    private serverStatusService: ServerStatusService,
    private dialog: MatDialog
  ) {}

  ngOnInit(): void {
    this.pluginSubscription = this.pluginService.plugins.pipe(
      map( plugins => plugins.filter( plugin => this.isPluginRequired(plugin)))
    ).subscribe( plugins => this.plugins = plugins);

    this.serverStatusSubscription = this.serverStatusService.serversStatus
      .pipe(
         map((status) => status.filter( status => this.isStatusRequired(status))),
      )
      .subscribe((status) => {
        this.status = status.find((el) => el !== undefined);
      });

    this.serverActionService.conditionChecks
      .pipe(
        map( status => status.filter( status => this.isCheckResultRequired(status))),
      )
      .subscribe((checkResults) => {
        this.conditionCheckResults = checkResults;
        this.allDependenciesMet = this.checkDependencies();
      });

      this.registerForMonitoring();
  }

  ngOnChanges(): void {
    this.allDependenciesMet = this.checkDependencies();
  }

  ngOnDestroy(): void {
    if (this.pluginSubscription) {
      this.pluginSubscription.unsubscribe();
    }
    if (this.serverStatusSubscription) {
      this.serverStatusSubscription.unsubscribe();
    }
    if (this.serverActionCheckSubscription) {
      this.serverActionCheckSubscription.unsubscribe();
    }
  }

  onClickAction() {
    if (!this.server || !this.guiAction) {
      return;
    }

    if (this.guiAction.needs_confirmation) {
      const message =
        "Do you want to execute the Action '" +
        this.guiAction.action.name +
        "' on server with IP " +
        this.server.ipaddress +
        '?';
      const confirmDialog = this.dialog.open(ConfirmDialogComponent, {
        data: {
          title: 'Confirm Action',
          message,
        },
      });
      confirmDialog.afterClosed().subscribe((result) => {
        if (result === true && this.server && this.guiAction) {
          this.serverActionService.executeAction(
            this.guiAction.feature.id,
            this.guiAction.action.id,
            this.server
          );
        }
      });
    } else {
      this.serverActionService.executeAction(
        this.guiAction.feature.id,
        this.guiAction.action.id,
        this.server
      );
    }
  }

  private filterStatus(status: Status[]): boolean {
    return status.find( s => this.isStatusRequired(s)) !== undefined;
  }
  private isStatusRequired(status: Status): boolean {
    return this.server !== undefined && this.server.ipaddress === status.ipaddress;
  }

  private filter(plugins: Plugin[]) : boolean {
    return plugins.find( p => this.isPluginRequired(p)) !== undefined;
  }
  private isPluginRequired(plugin: Plugin): boolean {
    return this.server !== undefined && this.server.features.find( f => f.id === plugin.id) !== undefined;
  }

  private filterCheckResults( results: ConditionCheckResult[]): boolean {
    return results.find( s => this.isCheckResultRequired(s)) !== undefined;
  }

  private isCheckResultRequired(result: ConditionCheckResult): boolean {
    return this.server !== undefined && this.server.ipaddress === result.ipaddress && this.guiAction?.feature.id === result.feature_id && this.guiAction.action.id === result.action_id;
  }

  private checkDependencies = (): boolean => {
    if (!this.guiAction || !this.guiAction.icon) {
      return false;
    }

    if (!this.statusDepencyMet()) {
      return false;
    }

    if (this.guiAction.action.depends === undefined || this.guiAction.action.depends.length === 0) {
      return true;
    } else {
      const foundResults = this.getAllDependenDataResults();

      if (foundResults && this.allDataDataReceived(foundResults)  && this.allDependenciesMetForData(foundResults)) {
        return true;
      }
    }
    return false;
  };

  private registerForMonitoring = () => {
    if (!this.server) {
      return;
    }
    for (const feature of this.server.features) {
      const plugin = this.plugins?.find( p => p.id === feature.id);
      if (plugin) {
        for (const action of plugin.actions) {
          if (action.depends && action.depends.length > 0) {
            this.serverActionService.registerFeatureActionOfServerForCheck(
              this.server,
              feature,
              action
            );
          }
        }
      }
    }
  };

  private statusDepencyMet = (): boolean | undefined => {
    return (
      this.guiAction &&
      this.status &&
      this.server &&
      ((this.guiAction.action.available_for_state === 'Any' &&
        this.status.is_running !== undefined) ||
        (this.guiAction.action.available_for_state === 'Active' &&
          this.status.is_running === true) ||
        (this.guiAction.action.available_for_state === 'Inactive' &&
          this.status.is_running === false))
    );
  };

  private allDependenciesMetForData = (foundResults: ConditionCheckResult[]) => {
    const result: boolean = foundResults
      .map((c) => c.result)
      .reduce((acc, cur) => acc && cur, true);

    return result;
  }

  private allDataDataReceived = (
    foundResults: ConditionCheckResult[] | undefined
  ) => {
    return (
      this.guiAction &&
      foundResults &&
      foundResults.length === this.guiAction.action.depends.length
    );
  }

  private getAllDependenDataResults = () => {
    return this.conditionCheckResults?.filter(
      (result) =>
        this.guiAction &&
        result.feature_id === this.guiAction.feature.id &&
        result.action_id === this.guiAction.action.id
    );
  }
}
