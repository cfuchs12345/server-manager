import { EventEmitter, Component, Input, Output } from '@angular/core';
import { Server } from 'src/app/services/servers/types';

@Component({
  selector: 'app-server-control-detail',
  templateUrl: './server-detail-control.component.html',
  styleUrls: ['./server-detail-control.component.scss'],
})
export class ServerDetailControlComponent {
  @Output() turnDetail = new EventEmitter<boolean>(false);

  @Input() server: Server | undefined = undefined;

  showBack: boolean = false;

  constructor() {}

  onClickTurnDetails = () => {
    this.showBack = !this.showBack;

    this.turnDetail.emit(this.showBack);
  };
}
