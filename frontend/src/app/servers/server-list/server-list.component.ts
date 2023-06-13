import {
  HostListener,
  Component,
  Input,
  OnChanges,
  SimpleChanges,
  ChangeDetectorRef,
} from '@angular/core';
import {
  animate,
  state,
  style,
  transition,
  trigger,
} from '@angular/animations';
import { MatTableDataSource } from '@angular/material/table';
import { RowData } from 'src/app/services/general/types';
import { Server } from 'src/app/services/servers/types';
import { AuthenticationService } from 'src/app/services/auth/authentication.service';

const initialDisplayedColumns: string[] = [
  'icons',
  'ipaddress',
  'name',
  'dnsname',
  'features',
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
export class ServerListComponent implements OnChanges {
  @Input() servers: Server[] = [];

  displayedColumns: string[] = initialDisplayedColumns.slice();
  isColumnsMobile: boolean = false; // if true, less columns are displayed for smaller screens

  showDetail: boolean = false;
  turnDetail: boolean = false;

  dataSource = new MatTableDataSource();
  expandedElement: RowData | null = null;

  applyFilter(event: Event) {
    this.dataSource.filter = (event.target as HTMLInputElement).value.trim();
  }

  constructor(
    private authService: AuthenticationService,
    private ref: ChangeDetectorRef
  ) {}

  ngOnChanges(changes: SimpleChanges): void {
    for (const propName in changes) {
      if (Object.hasOwn(changes, propName)) {
        switch (propName) {
          case 'servers': {
            this.dataSource.data = this.toRowData(this.servers);
            break;
          }
        }
      }
    }
  }

  onClickExpandRow = (rowData: RowData) => {
    let change = this.expandedElement !== rowData;

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

  private toRowData = (servers: Server[]): RowData[] => {
    var rowData: RowData[] = [];
    for (var server of servers) {
      rowData.push(
        new RowData(server, server.ipaddress, server.name, server.dnsname)
      );
    }
    return rowData;
  };
}
