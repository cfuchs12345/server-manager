import { Component, OnDestroy, OnInit } from '@angular/core';
import { FormControl, Validators } from '@angular/forms';
import { MatDialog, MatDialogRef } from '@angular/material/dialog';
import { Subscription } from 'rxjs';
import { ServerService } from 'src/app/services/servers/server.service';
import { Feature, Server, ServerFeature } from 'src/app/services/servers/types';
import { ConfirmDialogComponent } from 'src/app/ui/confirm-dialog/confirm-dialog.component';
import { ServerAddressType } from 'src/types/ServerAddress';
import { DeleteServerDialog } from '../dialog-delete-server';
import { Store } from '@ngrx/store';
import { selectAllServers, selectServerByIpAddress } from 'src/app/state/server/server.selectors';

@Component({
  selector: 'app-delete-server-modal',
  templateUrl: './delete-server-modal.component.html',
  styleUrls: ['./delete-server-modal.component.scss'],
})
export class DeleteServerModalComponent implements OnInit, OnDestroy {
  buttonTextDeleteServers = "Delete Server";
  buttonTextDeleteFeature = "Delete Feature";

  selectAll = false;
  loading = false;


  selectedServer: Server | undefined = undefined;
  selectedFeature: Feature | undefined = undefined;

  servers: Server[] = [];
  selectedServers: Server[] = [];
  serversWithFeatures: Server[] = [];
  features: Feature[] = [];

  subscriptionServers: Subscription | undefined = undefined;


  displayedColumns: string[] = ['delete', 'ipaddress', 'name'];

  constructor(private store: Store,private serverService: ServerService, private dialog: MatDialog, private ref: MatDialogRef<DeleteServerDialog>) {
  }

  ngOnInit(): void {
    this.loading = true;
    this.subscriptionServers  = this.store.select(selectAllServers).subscribe(
      (servers) => {
        if (servers) {
          this.servers = servers;
          this.serversWithFeatures = servers.filter( (server) => server.features.length > 0);
        } else {
          // clear messages when empty message received
          this.servers = [];
        }
        this.loading = false;
      }
    );
  }

  ngOnDestroy(): void {
    if( this.subscriptionServers ) {
      this.subscriptionServers.unsubscribe();
    }
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
      this.selectedServers.push(...this.servers);
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
      const ref = this;

      // cannot get it from store here, since we need the full data (features, credentials, params and so on)
      this.serverService.getServer(this.selectedServer.ipaddress, true).subscribe({
        next: (server) => {
          const filteredFeatures = server.features.filter( (feature) => feature.id !== ref.selectedFeature?.id);
          server.features = filteredFeatures;

          ref.serverService.updateServer(server);
        },
        error: (err) => {
        },
        complete: () => {
        }
      });
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
        this.serverService.deleteServers(this.selectedServers);
        this.ref.close();
      }
    });
  }

  onChangeServer = () => {
    this.features = this.selectedServer ? this.selectedServer.features : [];
  };

  onChangeFeature = () => {
  }

  isSelected = (server: Server) : boolean => {
    return this.selectedServers.find(s => s.ipaddress === server.ipaddress ) !== undefined;
  }

}
