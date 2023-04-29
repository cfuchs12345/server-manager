import {
  HostListener,
  Component,
  OnInit,
  Input,
  OnDestroy,
} from '@angular/core';
import { DomSanitizer, SafeHtml } from '@angular/platform-browser';
import { Subscription } from 'rxjs';
import { ServerService } from 'src/app/services/servers/server.service';
import { DataResult, Server, Status } from 'src/app/services/servers/types';

@Component({
  selector: 'app-server-control-detail',
  templateUrl: './server-detail-control.component.html',
  styleUrls: ['./server-detail-control.component.scss'],
})
export class ServerDetailControlComponent implements OnInit, OnDestroy {
  @Input() server: Server | undefined = undefined;

  showBack: boolean = false;

  constructor() {}

  ngOnInit(): void {
  }

  ngOnDestroy(): void {
  }

  onClickTurnDetails = () => {
    // TODO emit event
  };

}
