import {
  Component,
  Input,
  OnChanges,
  SimpleChanges,
  inject
} from '@angular/core';
import { Store } from '@ngrx/store';
import { Observable } from 'rxjs';
import { selectStatusByIpAddress } from 'src/app/state/status/status.selectors';
import { Server, Status } from 'src/app/services/servers/types';

@Component({
  selector: 'app-server-status',
  templateUrl: './server-status.component.html',
  styleUrls: ['./server-status.component.scss'],
})
export class ServerStatusComponent implements OnChanges {
  private store = inject(Store);

  @Input() server: Server | undefined = undefined;

  status$: Observable<Status | undefined> | undefined;


  ngOnChanges(changes: SimpleChanges): void {
    for (const propName in changes) {
      if (Object.hasOwn(changes, propName)) {
        switch (propName) {
          case 'server':
            if (this.server) {
              this.status$ = this.store.select(
                selectStatusByIpAddress(this.server.ipaddress)
              );
            }
        }
      }
    }
  }
}
