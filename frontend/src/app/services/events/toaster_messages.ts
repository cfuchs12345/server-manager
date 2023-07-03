import { Injectable } from '@angular/core';
import { Event } from './types';
import { ToastrService } from 'ngx-toastr';
import { Status } from '../servers/types';
import { Store } from '@ngrx/store';
import { Observable, of, map, take } from 'rxjs';
import { selectServerByIpAddress } from 'src/app/state/selectors/server.selectors';

enum Level {
  Success,
  Info,
  Warn,
  Error,
  None,
}

class ToasterPopupInfo {
  constructor(
    public message: string | Observable<string>,
    public title: string | Observable<string>,
    public level: Level
  ) {}
}

@Injectable({
  providedIn: 'root',
})
export class ToasterPopupGenerator {
  constructor(private store: Store, private toasterService: ToastrService) {}

  handleEvent(event: Event) {
    this.show(this.getToasterPopupInfo(event));
  }

  getToasterPopupInfo = (event: Event): ToasterPopupInfo | null => {
    switch (event.object_type) {
      case 'Server':
        return this.getServerInfo(event);
      case 'ConditionCheckResult':
        return this.getConditionInfo(event);
      case 'Status':
        return this.getStatusInfo(event);
    }
    return null;
  };

  show = (eventInfo: ToasterPopupInfo | null) => {
    if (eventInfo === null) {
      return;
    }

    const message =
      typeof eventInfo.message === 'string'
        ? of(eventInfo.message)
        : eventInfo.message;

    const title =
      typeof eventInfo.title === 'string'
        ? of(eventInfo.title)
        : eventInfo.title;

    message.pipe(take(1)).subscribe((message) => {
      title.pipe(take(1)).subscribe((title) => {
        switch (eventInfo.level) {
          case Level.Success:
            this.toasterService.success(message, title);
            break;
          case Level.Info:
            this.toasterService.info(message, title);
            break;
          case Level.Warn:
            this.toasterService.warning(message, title);
            break;
          case Level.Error:
            this.toasterService.error(message, title);
            break;
        }
      });
    });
  };

  getConditionInfo = (event: Event): ToasterPopupInfo | null => {
    return null;
  };

  getStatusInfo = (event: Event): ToasterPopupInfo | null => {
    switch (event.event_type) {
      case 'Update': {
        const status: Status = JSON.parse(event.value);

        const server$ = this.store.select(
          selectServerByIpAddress(status.ipaddress)
        );

        if (!status.is_running) {
          return new ToasterPopupInfo(
            server$.pipe(
              map(
                (server) =>
                  `Server ${server?.name} ${server?.dnsname} went down`
              )
            ),
            server$.pipe(map((server) => `Server ${server?.ipaddress}`)),
            Level.Warn
          );
        } else {
          return new ToasterPopupInfo(
            server$.pipe(
              map(
                (server) => `Server ${server?.name} ${server?.dnsname} came up`
              )
            ),
            server$.pipe(map((server) => `Server ${server?.ipaddress}`)),
            Level.Success
          );
        }
      }
      case 'Delete': {
        return new ToasterPopupInfo(
          'Server status is unknown',
          'Server',
          Level.Warn
        );
      }
    }
    return null;
  };

  getServerInfo = (event: Event): ToasterPopupInfo | null => {
    const server$ = this.store.select(selectServerByIpAddress(event.key));

    switch (event.event_type) {
      case 'Insert': {
        return new ToasterPopupInfo(
          server$.pipe(
            map(
              (server) =>
                server !== undefined ? `New Server ${server?.name} ${server?.dnsname} has been added` : `New Server ${event.key} has been added`

            )
          ),
          server$.pipe(map(() =>  `Server ${event.key}`)),
          Level.Info
        );
      }
      case 'Update': {
        return new ToasterPopupInfo(
          server$.pipe(
            map(
              (server) =>
                `New Server ${server?.name} ${server?.dnsname} has been updated`
            )
          ),
          server$.pipe(map((server) => `Server ${server?.ipaddress}`)),
          Level.Info
        );
      }
      case 'Delete': {
        return new ToasterPopupInfo(
          `Server ${event.key} has been removed`,
          `Server ${event.key}`,
          Level.Warn
        );
      }
    }
  };
}
