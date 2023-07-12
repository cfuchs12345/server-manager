import { Injectable } from '@angular/core';
import { Event } from './types';
import { ToastrService } from 'ngx-toastr';
import {
  ConditionCheckResult,
  Status,
} from '../servers/types';
import { Store } from '@ngrx/store';
import { Observable, of, map, take } from 'rxjs';
import { selectServerByIpAddress } from 'src/app/state/server/server.selectors';

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

const OPTIONS = {
  timeOut: 30000,
  extendedTimeOut: 1000,
  progressBar: true,
  tapToDismiss: true,
};

@Injectable({
  providedIn: 'root',
})
export class ToasterPopupGenerator {
  constructor(private store: Store, private toasterService: ToastrService) {}

  handleEvent(eventAndObject: [Event, any]) {
    this.show(this.getToasterPopupInfo(eventAndObject[0]));
  }

  getToasterPopupInfo = (event: Event): ToasterPopupInfo[] => {
    switch (event.object_type) {
      case 'Server':
        return this.getServerInfo(event);
      case 'ConditionCheckResult':
        return this.getConditionInfo(event);
      case 'Status':
        return this.getStatusInfo(event);
    }
    return [];
  };

  show = (eventInfos: ToasterPopupInfo[]) => {
    if (eventInfos.length === 0) {
      return;
    }

    for (const eventInfo of eventInfos) {
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
              this.toasterService.success(message, title, OPTIONS);
              break;
            case Level.Info:
              this.toasterService.info(message, title, OPTIONS);
              break;
            case Level.Warn:
              this.toasterService.warning(message, title, OPTIONS);
              break;
            case Level.Error:
              this.toasterService.error(message, title, OPTIONS);
              break;
          }
        });
      });
    }
  };

  getConditionInfo = (event: Event): ToasterPopupInfo[] => {
    const vec: ToasterPopupInfo[] = [];

    const server$ = this.store.select(selectServerByIpAddress(event.key));

    switch (event.event_type) {
      case 'Update':
        {
          const cr: ConditionCheckResult = JSON.parse(event.value);

          for (const sr of cr.subresults) {
            if (sr.result) {
              vec.push(
                new ToasterPopupInfo(
                  server$.pipe(
                    map(
                      () =>
                        `Condition for action changed. Action '${sr.action_id}' is available.`
                    )
                  ),
                  server$.pipe(
                    map(
                      (server) =>
                        `Action Condition for Server ${server?.ipaddress}`
                    )
                  ),
                  Level.Success
                )
              );
            } else {
              vec.push(
                new ToasterPopupInfo(
                  server$.pipe(
                    map(
                      () =>
                        `Condition for action changed. Action '${sr.action_id}' is not available.`
                    )
                  ),
                  server$.pipe(
                    map(
                      (server) =>
                        `Action Condition for Server ${server?.ipaddress}`
                    )
                  ),
                  Level.Warn
                )
              );
            }
          }
        }
        break;
    }
    return vec;
  };

  getStatusInfo = (event: Event): ToasterPopupInfo[] => {
    const vec: ToasterPopupInfo[] = [];

    const server$ = this.store.select(selectServerByIpAddress(event.key));

    switch (event.event_type) {
      case 'Update':
        {
          const status: Status = JSON.parse(event.value);

          if (!status.is_running) {
            vec.push(
              new ToasterPopupInfo(
                server$.pipe(
                  map(
                    (server) =>
                      `Server ${server?.name} ${server?.dnsname} went down`
                  )
                ),
                server$.pipe(map((server) => `Server ${server?.ipaddress}`)),
                Level.Warn
              )
            );
          } else {
            vec.push(
              new ToasterPopupInfo(
                server$.pipe(
                  map(
                    (server) =>
                      `Server ${server?.name} ${server?.dnsname} came up`
                  )
                ),
                server$.pipe(map((server) => `Server ${server?.ipaddress}`)),
                Level.Success
              )
            );
          }
        }
        break;
      case 'Delete':
        {
          vec.push(
            new ToasterPopupInfo(
              'Server status is unknown',
              'Server',
              Level.Warn
            )
          );
        }
        break;
    }
    return vec;
  };

  getServerInfo = (event: Event): ToasterPopupInfo[] => {
    const vec: ToasterPopupInfo[] = [];

    const server$ = this.store.select(selectServerByIpAddress(event.key));

    switch (event.event_type) {
      case 'Insert':
        {
          vec.push(
            new ToasterPopupInfo(
              server$.pipe(
                map((server) =>
                  server !== undefined
                    ? `New Server ${server?.name} ${server?.dnsname} has been added`
                    : `New Server ${event.key} has been added`
                )
              ),
              server$.pipe(map(() => `Server ${event.key}`)),
              Level.Success
            )
          );
        }
        break;
      case 'Update':
        {
          vec.push(
            new ToasterPopupInfo(
              server$.pipe(
                map(
                  (server) =>
                    `Server ${server?.name} ${server?.dnsname} has been updated`
                )
              ),
              server$.pipe(map((server) => `Server ${server?.ipaddress}`)),
              Level.Info
            )
          );
        }
        break;
      case 'Delete':
        {
          vec.push(
            new ToasterPopupInfo(
              `Server ${event.key} has been removed`,
              `Server ${event.key}`,
              Level.Warn
            )
          );
        }
        break;
    }
    return vec;
  };
}
