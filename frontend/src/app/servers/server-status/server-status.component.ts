import { Component, OnInit, Input } from '@angular/core';
import { Store } from '@ngrx/store';
import { Subscription } from 'rxjs';
import { selectStatusByIpAddress } from 'src/app/state/selectors/status.selectors';
import { Server, Status } from 'src/app/services/servers/types';

@Component({
  selector: 'app-server-status',
  templateUrl: './server-status.component.html',
  styleUrls: ['./server-status.component.scss'],
})
export class ServerStatusComponent implements OnInit {
  @Input() server: Server | undefined = undefined;
  private status: Status | undefined = undefined;

  private serverStatusSubscription: Subscription | undefined = undefined;

  constructor(private store: Store) {}

  ngOnInit(): void {
    if( this.server ) {
      this.serverStatusSubscription = this.store.select(selectStatusByIpAddress(this.server.ipaddress)).subscribe((status) => {
        this.status = status;
      });
    }
  }

  ngOnDestroy(): void {
    if (this.serverStatusSubscription) {
      this.serverStatusSubscription.unsubscribe();
    }
  }

  isRunning = (): boolean => {
    if (this.status) {
      return this.status.is_running;
    }
    return false;
  };
}
