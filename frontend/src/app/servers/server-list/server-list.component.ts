import {
  HostListener,
  Component,
  Input,
  OnChanges,
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

  constructor() {}

  ngOnChanges(): void {
      this.dataSource.data = this.toRowData(this.servers);
  }


  expandRow = (rowData: RowData) => {
    this.expandedElement = this.expandedElement === rowData ? null : rowData;


    rowData.show_details = !rowData.show_details;
  };

  turnDetailChange(event: boolean) {
    this.turnDetail = event;
  }


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
