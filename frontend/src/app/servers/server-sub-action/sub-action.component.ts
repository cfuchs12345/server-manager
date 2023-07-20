import { Component, NgZone, OnInit, inject } from '@angular/core';
import { MatDialog } from '@angular/material/dialog';
import { ConfirmDialogComponent } from 'src/app/ui/confirm-dialog/confirm-dialog.component';
import { ErrorService, Source } from 'src/app/services/errors/error.service';
import { Store } from '@ngrx/store';
import { selectPluginById } from 'src/app/state/plugin/plugin.selectors';
import { executeAction } from 'src/app/state/action/action.actions';

@Component({
    selector: 'app-server-sub-action',
    templateUrl: './server-sub-action.component.html',
    styleUrls: ['./server-sub-action.component.scss'],
    standalone: true,
})
export class ServerSubActionComponent implements OnInit {
  private store = inject(Store);
  private errorService = inject(ErrorService);
  private dialog = inject(MatDialog);
  private zone = inject(NgZone);

  ngOnInit() {
    if (!window.MyServerManagerNS.executeSubAction) {
      window.MyServerManagerNS.executeSubAction = (
        feature_id: string,
        action_id: string,
        action_name: string,
        data_id: string,
        action_params: string,
        ipaddress: string
      ) => {
        // eslint-disable-next-line  @rx-angular/no-zone-run-apis
        this.zone.run(() => {
          this.executeSubAction(
            feature_id,
            action_id,
            action_name,
            data_id,
            action_params,
            ipaddress
          );
        });
      };
    }
  }

  executeSubAction = (
    feature_id: string,
    action_id: string,
    action_name: string,
    data_id: string,
    action_params: string,
    ipaddress: string
  ) => {
    const pluginById$ = this.store.select(selectPluginById(feature_id));

    pluginById$.subscribe((plugin) => {
      if (plugin) {
        const action = plugin.actions.find((a) => a.id === action_id);
        if (action) {
          if (action.needs_confirmation) {
            const message =
              "Do you want to execute the Action '" +
              action_name +
              "' on server with IP " +
              ipaddress +
              '?';
            const confirmDialog = this.dialog.open(ConfirmDialogComponent, {
              data: {
                title: 'Confirm Action',
                message,
              },
            });
            confirmDialog.afterClosed().subscribe((result) => {
              if (result === true) {
                this.store.dispatch(
                  executeAction({
                    feature_id,
                    action_id,
                    ipaddress,
                    action_params,
                  })
                );
              }
            });
          } else {
            this.store.dispatch(
              executeAction({ feature_id, action_id, ipaddress, action_params })
            );
          }
        } else {
          this.errorService.newError(
            Source.ServerSubActionComponent,
            ipaddress,
            'Could not execute sub-action ' +
              action_id +
              ' since plugin ' +
              feature_id +
              " doesn't contain such an action"
          );
        }
      } else {
        this.errorService.newError(
          Source.ServerSubActionComponent,
          ipaddress,
          'Could not execute sub-action ' +
            action_id +
            ' since plugin ' +
            feature_id +
            ' is not known'
        );
      }
    });
  };
}
