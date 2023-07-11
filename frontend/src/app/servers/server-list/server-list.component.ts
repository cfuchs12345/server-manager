import {
  HostListener,
  Component,
  Input,
  OnInit,
  OnDestroy,
  ViewChild
} from '@angular/core';
import {
  animate,
  state,
  style,
  transition,
  trigger,
} from '@angular/animations';
import { MatTable, MatTableDataSource } from '@angular/material/table';
import { RowData } from 'src/app/services/general/types';
import { Server } from 'src/app/services/servers/types';
import { AuthenticationService } from 'src/app/services/auth/authentication.service';
import { Observable } from 'rxjs';
import { SubscriptionHandler } from 'src/app/shared/subscriptionHandler';

const initialDisplayedColumns: string[] = [
  'icons',
  'ipaddress',
  'name',
  'dnsname',
  'features',
  'notifications',
  'actions',
];
const displayedColumnForMobilesPhones: string[] = [
  'icons',
  'ipaddress',
  'name',
  'actions',
];

@Component({
  selector: 'app-server-list',
  templateUrl: './server-list.component.html',
  styleUrls: ['./server-list.component.scss'],
  animations: [
    trigger('detailExpand', [
      state('collapsed', style({ height: '0px' })),
      state('expanded', style({ height: '*' })),
      transition('collapsed <=> expanded', animate('0.2s')),
    ]),
  ],
})
export class ServerListComponent implements OnInit, OnDestroy {
  @Input() servers$: Observable<Server[]> | undefined;
  @ViewChild('serversTable', {static:true}) table: MatTable<RowData> | undefined;

  displayedColumns: string[] = initialDisplayedColumns.slice();
  isColumnsMobile = false; // if true, less columns are displayed for smaller screens

  showDetail = false;
  turnDetail = false;

  dataSource = new MatTableDataSource();
  expandedElement: RowData | null = null;

  private subscriptionHandler = new SubscriptionHandler(this);
  constructor(private authService: AuthenticationService) {}

  ngOnInit(): void {
    if (this.servers$ && this.servers$) {
      this.subscriptionHandler.subscription = this.servers$.subscribe(
        (servers) => this.toRowData(servers)
      );
    }
  }

  ngOnDestroy(): void {
    this.subscriptionHandler.onDestroy();
  }

  applyFilter(event: Event) {
    this.dataSource.filter = (event.target as HTMLInputElement).value.trim();
  }

  onClickExpandRow = (rowData: RowData) => {
    const change = this.expandedElement !== rowData;

    // same detail clicked again - will close the details, so we set the element to null
    if (!change) {
      rowData.show_details = !rowData.show_details;
      this.expandedElement = null;
    } else {
      this.expandedElement = rowData;
    }

    this.turnDetail = false;
  };

  onClickLogout = () => {
    this.authService.logout();
  };

  onClickTurnDetailChange = (event: boolean) => {
    this.turnDetail = event;
  };

  @HostListener('window:resize', ['$event'])
  onResize(event: UIEvent) {
    const target = event.target as Window;

    if (!event.target || !target.innerWidth) {
      return;
    }

    if (target.innerWidth < 600 && !this.isColumnsMobile) {
      this.displayedColumns = displayedColumnForMobilesPhones.slice();
      this.isColumnsMobile = true;
    } else if (target.innerWidth >= 600 && this.isColumnsMobile) {
      this.displayedColumns = initialDisplayedColumns.slice();
      this.isColumnsMobile = false;
    }
  }

  private toRowData = (servers: Server[]) => {
    if (this.dataSource.data === undefined || this.dataSource.data.length === 0) {
      const rowData: RowData[] = [];
      for (const server of servers) {
        rowData.push(this.createRowDataForServer(server));
      }
      this.dataSource.data = rowData;
    } else {
      let updated = this.removeServersIfNecessary(servers);
      updated = updated || this.addOrUpdateServers(servers);
      if( updated ) {
        if( this.table ) {
          this.table.renderRows();
        }
      }
    }
  };

  private removeServersIfNecessary = (servers: Server[]): boolean => {
    const dataArray = this.dataSource.data as RowData[];
    let updated = false;
    for (const [index, dataRow] of dataArray.entries()) {
      if (
        servers.find((server) => server.ipaddress === dataRow.ipaddress) ===
        undefined
      ) {
        this.dataSource.data.splice(index);
        updated = true;
      }
    }
    return updated;
  };

  private addOrUpdateServers = (servers: Server[]): boolean => {
    let updated = false;
    for (const server of servers) {
      const [index, existing] = this.getExisting(server);
      const newRowData = this.createRowDataForServer(server);
      if( index === -1) {
        this.dataSource.data.splice(index, 1);
        updated = true;
      }
      else if (index !== -1 && this.different(existing, newRowData)) { // if update - else only add
        this.dataSource.data.splice(index, 1, newRowData);
        updated = true;
      }
    }
    return updated;
  };

  private getExisting = (server: Server): [number, RowData | null] => {
    const dataArray = this.dataSource.data as RowData[];
    for (const [index, existingData] of dataArray.entries()) {
      if (existingData.ipaddress === server.ipaddress) {
        return [index, existingData];
      }
    }
    return [-1, null];
  };

  private createRowDataForServer = (server: Server): RowData => {
    return new RowData(server, server.ipaddress, server.name, server.dnsname, server.version);
  }

  private different = (existingRowData: RowData | null, newRowData: RowData) : boolean => {
    if( existingRowData !== null) {
      return existingRowData.version !== newRowData.version;
    }
    return false;
  }
}


