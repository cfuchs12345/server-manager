import { Component, OnDestroy, OnInit } from '@angular/core';
import { FormControl, Validators } from '@angular/forms';
import { MatDialog, MatDialogRef } from '@angular/material/dialog';
import { Subscription } from 'rxjs';
import { ServerService } from 'src/app/services/servers/server.service';
import { Feature, Server, ServerFeature } from 'src/app/services/servers/types';
import { ConfirmDialogComponent } from 'src/app/ui/confirm-dialog/confirm-dialog.component';
import { ServerAddressType } from 'src/types/ServerAddress';
import { DeleteServerDialog } from '../dialog-delete-server';

@Component({
  selector: 'app-delete-server-modal',
  templateUrl: './delete-server-modal.component.html',
  styleUrls: ['./delete-server-modal.component.scss'],
})
export class DeleteServerModalComponent implements OnInit, OnDestroy {
  buttonTextDeleteServers: string = "Delete Server";
  buttonTextDeleteFeature: string = "Delete Feature";

  selectAll: boolean = false;
  loading: boolean = false;


  selectedServer: Server | undefined = undefined;
  selectedFeature: Feature | undefined = undefined;

  servers: Server[] = [];
  serversWithFeatures: Server[] = [];
  features: Feature[] = [];

  subscriptionServers: Subscription | undefined = undefined;


  displayedColumns: string[] = ['delete', 'ipaddress', 'name'];

  constructor(private serverService: ServerService, private dialog: MatDialog, private ref: MatDialogRef<DeleteServerDialog>) {
  }

  ngOnInit(): void {
    this.loading = true;
    this.subscriptionServers = this.serverService.servers.subscribe((servers) => {
      if (servers) {
        this.servers = servers;
        this.serversWithFeatures = servers.filter( (server) => server.features.length > 0);
      } else {
        // clear messages when empty message received
        this.servers = [];
      }
      this.loading = false;
    });
    this.serverService.listServers();
  }

  ngOnDestroy(): void {
    if( this.subscriptionServers ) {
      this.subscriptionServers.unsubscribe();
    }
  }

  serversSelected = (): number => {
    const count = this.servers.filter((server) => server.selected === true).length;

    this.buttonTextDeleteServers = count <= 1 ? 'Delete Server' : 'Delete Servers (#' + count + ")"

    return count;
  }

  onClickSelectAll = () => {
    this.selectAll = !this.selectAll;


    this.servers.forEach( (server) => server.selected = this.selectAll);
  }

  onClickSelectServer = (server: Server) => {
    server.selected = !server.selected;
  }

  removeFeatureFromServer = () => {
    if(this.selectedServer && this.selectedFeature) {
      const features = this.selectedServer.features.filter( (feature) => feature.id !== this.selectedFeature?.id);

      const featuresOfServer = new ServerFeature(this.selectedServer.ipaddress, features, true);

      this.serverService.updateServerFeatures([featuresOfServer]);
    }
  }

  onClickDeleteServers = () => {
    const serversToDelete = this.servers.filter((server) => server.selected === true);
    const message = serversToDelete.length > 1 ? "Do you really want to delete " +serversToDelete.length + " servers?" : "Do you really want to delete the server: "  + serversToDelete[0].ipaddress + "?";
    const confirmDialog = this.dialog.open(ConfirmDialogComponent, {
      data: {
        title: 'Confirm Server Deletion',
        message
      }
    });
    confirmDialog.afterClosed().subscribe(result => {
      if (result === true) {
        this.serverService.deleteServers(serversToDelete);
        this.ref.close();
      }
    });
  }

  onChangeServer = () => {
    this.features = this.selectedServer ? this.selectedServer.features : [];
  };

  onChangeFeature = () => {
  }

}
