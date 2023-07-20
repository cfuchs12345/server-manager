import { EventEmitter, Component, Input, Output } from '@angular/core';
import { Server } from 'src/app/services/servers/types';
import { MatButtonModule } from '@angular/material/button';

@Component({
    selector: 'app-server-control-detail',
    templateUrl: './server-detail-control.component.html',
    styleUrls: ['./server-detail-control.component.scss'],
    standalone: true,
    imports: [MatButtonModule],
})
export class ServerDetailControlComponent {
  @Output() turnDetail = new EventEmitter<boolean>(false);

  @Input() server: Server | undefined = undefined;

  showBack = false;

  onClickTurnDetails = () => {
    this.showBack = !this.showBack;

    this.turnDetail.emit(this.showBack);
  };
}
