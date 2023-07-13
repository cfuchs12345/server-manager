import { Component, OnDestroy } from '@angular/core';
import { MatDialog, MatDialogRef } from '@angular/material/dialog';
import { Observable } from 'rxjs';
import { ServerService } from 'src/app/services/servers/server.service';
import { Feature, Server, ServerFeature } from 'src/app/services/servers/types';
import { ConfirmDialogComponent } from 'src/app/ui/confirm-dialog/confirm-dialog.component';
import { DeleteServerDialog } from '../dialog-delete-server';
import { Store } from '@ngrx/store';
import { selectAllServers, selectAllServersWithFeatures } from 'src/app/state/server/server.selectors';
import { SubscriptionHandler } from 'src/app/shared/subscriptionHandler';
import { removeServerFeature, removeServers } from 'src/app/state/server/server.actions';

@Component({
  selector: 'app-delete-server-modal',
  templateUrl: './delete-server-modal.component.html',
  styleUrls: ['./delete-server-modal.component.scss'],
})
export class DeleteServerModalComponent implements OnDestroy {
  buttonTextDeleteServers = "Delete Server";
  buttonTextDeleteFeature = "Delete Feature";

  selectAll = false;
  loading = false;


  selectedServer: Server | undefined = undefined; // for removal of feature
  selectedFeature: Feature | undefined = undefined;// for removal of feature

  servers$: Observable<Server[]>;
  serversWithFeatures$: Observable<Server[]>;

  selectedServers: Server[] = [];
  features: Feature[] = [];

  subscriptionHandler = new SubscriptionHandler(this);

  displayedColumns: string[] = ['delete', 'ipaddress', 'name'];

  constructor(private store: Store,private serverService: ServerService, private dialog: MatDialog, private ref: MatDialogRef<DeleteServerDialog>) {
    this.servers$ = this.store.select(selectAllServers);
    this.serversWithFeatures$ = this.store.select(selectAllServersWithFeatures);
  }


  ngOnDestroy(): void {
    this.subscriptionHandler.onDestroy();
  }

  serversSelected = (): number => {
    const count = this.selectedServers.length;

    this.buttonTextDeleteServers = count <= 1 ? 'Delete Server' : 'Delete Servers (#' + count + ")"

    return count;
  }

  onClickSelectAll = () => {
    this.selectAll = !this.selectAll;

    this.selectedServers.splice(0, this.selectedServers.length);

    if( this.selectAll ) {
      this.subscriptionHandler.subscription = this.servers$.subscribe( (servers) => this.selectedServers.push(... servers));

    }
  }

  onClickSelectServer = (server: Server) => {
    const index = this.selectedServers.indexOf(server);

    if( index > -1) {
      this.selectedServers.splice(index);
    }
    else {
      this.selectedServers.push(server);
    }
  }

  removeFeatureFromServer = () => {
    if(this.selectedServer && this.selectedFeature) {
      const serverFeature = new ServerFeature(this.selectedServer.ipaddress, [this.selectedFeature]);
      this.store.dispatch(removeServerFeature({serverFeature}));

      this.selectedFeature = undefined;
      this.selectedServer = undefined;
    }
  }

  onClickDeleteServers = () => {
    const message = this.selectedServers.length > 1 ? "Do you really want to delete " +this.selectedServers.length + " servers?" : "Do you really want to delete the server: "  + this.selectedServers[0].ipaddress + "?";
    const confirmDialog = this.dialog.open(ConfirmDialogComponent, {
      data: {
        title: 'Confirm Server Deletion',
        message
      }
    });
    confirmDialog.afterClosed().subscribe(result => {
      if (result === true) {
        this.store.dispatch(removeServers({servers: this.selectedServers}));
        this.ref.close();
      }
    });
  }

  onChangeServer = () => {
    this.features = this.selectedServer ? this.selectedServer.features : [];
  };


  isSelected = (server: Server) : boolean => {
    return this.selectedServers.find(s => s.ipaddress === server.ipaddress ) !== undefined;
  }
}
