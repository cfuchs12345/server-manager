import { Injectable } from '@angular/core';
import { Subject } from 'rxjs';
import { Error } from './types';
import { NGXLogger } from 'ngx-logger';

export enum Source {
  AuthenticationService,
  GeneralService,
  UserService,
  ServerService,
  ServerStatusService,
  ServerDiscoveryService,
  ServerDataService,
  ServerActionService,
  MonitoringService,
  PluginService,
  ServerSubActionComponent,
  ServerFeaturesComponent,
  AutodiscoverServerModalComponent,
  NotificationService,
  EventService
}

@Injectable()
export class ErrorService {
  private _errors = new Subject<Error>();
  readonly errors = this._errors.asObservable();

  constructor(private logger: NGXLogger) {}

  newError(
    source: Source,
    ipaddress: string | undefined = undefined,
    error_object: any
  ) {
    let text = 'Unkown';
    if (error_object) {
      if (Object.hasOwn(error_object, 'error')) {
        if (error_object.error && Object.hasOwn(error_object.error, 'error')) {
          text = error_object.error.error;
        } else {
          text = error_object.error;
        }
      } else if (Object.hasOwn(error_object, 'message')) {
        text = error_object.message;
      } else {
        text = JSON.stringify(error_object);
      }
    }
    const error = new Error(source, ipaddress, text, new Date());

    this.logger.trace("error", error);
    this._errors.next(error);
  }
}
