import {
  HostListener,
  Component,
  OnInit,
  Input,
  OnDestroy,
  OnChanges,
} from '@angular/core';
import { DomSanitizer, SafeHtml } from '@angular/platform-browser';
import { Subscription } from 'rxjs';
import { ServerDataService } from 'src/app/services/servers/server-data.service';
import { ServerService } from 'src/app/services/servers/server.service';
import { DataResult, Server, Status } from 'src/app/services/servers/types';

@Component({
  selector: 'app-server-detail',
  templateUrl: './server-detail.component.html',
  styleUrls: ['./server-detail.component.scss'],
})
export class ServerDetailComponent implements OnChanges, OnDestroy {
  @Input() server: Server | undefined = undefined;
  @Input() showDetail: boolean = false;
  @Input() turnDetail: boolean = false;

  dataResults: Map<String, DataResult> = new Map();
  dataResultSubscription: Subscription | undefined = undefined;




  constructor(
    private sanitizer: DomSanitizer,
    private serverDataService: ServerDataService
  ) {}

  ngOnChanges(): void {
    if (this.showDetail) {
      this.dataResultSubscription = this.serverDataService.dataResults.subscribe(
        (result) => {
          this.dataResults = result;
        }
      );

      if (this.server) {
        this.serverDataService.queryData(this.server);
      }
    } else {
      if (this.dataResultSubscription) {
        this.dataResultSubscription.unsubscribe();
      }
    }
  }

  ngOnDestroy(): void {
    if( this.dataResultSubscription ) {
      this.dataResultSubscription.unsubscribe();
    }
  }


  getDataResult = (): SafeHtml | undefined => {
    if (!this.server) {
      return undefined;
    }

    var result = this.dataResults.get(this.server.ipaddress);
    if (result) {
      var concatString: string = result.results.join('');

      return this.sanitizer.bypassSecurityTrustHtml(concatString);
    }

    return this.sanitizer.bypassSecurityTrustHtml('');
  };
}
