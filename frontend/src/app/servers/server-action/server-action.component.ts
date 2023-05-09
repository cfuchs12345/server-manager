import { Component, OnInit, Input, OnDestroy, OnChanges } from '@angular/core';
import { MatDialog } from '@angular/material/dialog';
import { Subscription, map } from 'rxjs';
import { GUIAction } from 'src/app/services/general/types';
import {
  ConditionCheckResult,
  Server,
  Status,
} from 'src/app/services/servers/types';
import { ConfirmDialogComponent } from 'src/app/ui/confirm-dialog/confirm-dialog.component';
import { ServerActionService } from 'src/app/services/servers/server-action.service';

@Component({
  selector: 'app-server-action',
  templateUrl: './server-action.component.html',
  styleUrls: ['./server-action.component.scss'],
})
export class ServerActionComponent implements OnInit, OnDestroy, OnChanges {
  @Input() server: Server | undefined = undefined;
  @Input() guiAction: GUIAction | undefined = undefined;
  @Input() status: Status | undefined = undefined;

  allDependenciesMet: boolean = false;

  private conditionCheckResult: ConditionCheckResult | undefined = undefined;

  private serverActionCheckSubscription: Subscription | undefined = undefined;

  constructor(
    private serverActionService: ServerActionService,
    private dialog: MatDialog
  ) {}

  ngOnInit(): void {
    this.serverActionService.conditionChecks
      .pipe(
        map((status) =>
          status.filter((status) => this.isCheckResultRequired(status))
        )
      )
      .subscribe((checkResults) => {
        this.conditionCheckResult = checkResults.find((res) => res);
        this.allDependenciesMet =
          this.conditionCheckResult !== undefined &&
          this.conditionCheckResult.result;
      });
  }

  ngOnChanges(): void {
    this.allDependenciesMet =
      this.conditionCheckResult !== undefined &&
      this.conditionCheckResult.result;
  }

  ngOnDestroy(): void {
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
            this.server.ipaddress
          );
        }
      });
    } else {
      this.serverActionService.executeAction(
        this.guiAction.feature.id,
        this.guiAction.action.id,
        this.server.ipaddress
      );
    }
  }

  private isCheckResultRequired(result: ConditionCheckResult): boolean {
    return (
      this.server !== undefined &&
      this.server.ipaddress === result.ipaddress &&
      this.guiAction?.feature.id === result.feature_id &&
      this.guiAction.action.id === result.action_id
    );
  }
}
