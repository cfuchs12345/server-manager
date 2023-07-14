import { Component, Input, OnChanges, ChangeDetectorRef } from '@angular/core';
import { MatDialog } from '@angular/material/dialog';
import { GUIAction } from 'src/app/services/general/types';
import {
  ConditionCheckResult,
  Server,
  Status,
} from 'src/app/services/servers/types';
import { ConfirmDialogComponent } from 'src/app/ui/confirm-dialog/confirm-dialog.component';
import { Store } from '@ngrx/store';
import { executeAction } from 'src/app/state/action/action.actions';

@Component({
  selector: 'app-server-action',
  templateUrl: './server-action.component.html',
  styleUrls: ['./server-action.component.scss'],
})
export class ServerActionComponent implements OnChanges {
  @Input() server: Server | undefined = undefined;
  @Input() guiAction: GUIAction | undefined = undefined;
  @Input() status: Status | undefined = undefined;
  @Input() conditionCheckResult: ConditionCheckResult | undefined = undefined;

  allDependenciesMet = false;

  constructor(
    private store: Store,
    private dialog: MatDialog,
    private cdr: ChangeDetectorRef
  ) {}

  ngOnChanges(): void {
    const old = this.allDependenciesMet;
    this.allDependenciesMet = this.allDependenciesMetCheck();

    if (old !== this.allDependenciesMet) {
      this.cdr.detectChanges();  // eslint-disable-line @rx-angular/no-explicit-change-detection-apis
    }
  }

  allDependenciesMetCheck = (): boolean => {
    if (
      this.guiAction !== undefined &&
      this.conditionCheckResult &&
      this.server &&
      this.conditionCheckResult.ipaddress === this.server.ipaddress
    ) {
      const foundSubResult = this.conditionCheckResult.subresults.find(
        (sr) =>
          sr.feature_id === this.guiAction?.feature.id &&
          sr.action_id === this.guiAction.action.id
      );

      return foundSubResult !== undefined ? foundSubResult.result : false;
    }
    return false;
  };

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
          this.store.dispatch(
            executeAction({
              feature_id: this.guiAction.feature.id,
              action_id: this.guiAction.action.id,
              ipaddress: this.server.ipaddress,
              action_params: undefined
            })
          );
        }
      });
    } else {
      this.store.dispatch(
        executeAction({
          feature_id: this.guiAction.feature.id,
          action_id: this.guiAction.action.id,
          ipaddress: this.server.ipaddress,
          action_params: undefined
        })
      );
    }
  }
}
