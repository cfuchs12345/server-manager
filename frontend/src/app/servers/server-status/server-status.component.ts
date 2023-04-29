import { HostListener, Component, OnInit, Input  } from '@angular/core';
import { Subscription, filter, map, tap } from 'rxjs';
import { ServerStatusService } from 'src/app/services/servers/server-status.service';
import { ServerService } from 'src/app/services/servers/server.service';
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

  constructor(private serverStatusService: ServerStatusService) { }

  ngOnInit(): void {
    this.serverStatusSubscription = this.serverStatusService.serversStatus.pipe(
      //tap( (status) => console.log("before filter " + status.length)),
      map( status => {
        return status.filter( s =>   this.server && s.ipaddress === this.server.ipaddress)
      }),
      //tap( (status) => console.log( "after filter " + status.length)),
    ).subscribe( status => {
      this.status = status.find(el => el !== undefined)
    });
  }

  ngOnDestroy(): void {
    if (this.serverStatusSubscription) {
      this.serverStatusSubscription.unsubscribe();
    }
  }

  isRunning = (): boolean => {
    if( this.status && this.status.is_running) {
      return this.status.is_running;
    }
    return false;
  }
}
