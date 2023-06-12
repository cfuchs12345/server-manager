import { Injectable } from '@angular/core';
import { BehaviorSubject } from 'rxjs';
import { Error } from './types';

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
}

@Injectable()
export class ErrorService {
  private _errors = new BehaviorSubject<Map<string, Error>>(new Map());
  readonly errors = this._errors.asObservable();

  private dataStore: {
    errors: Map<string, Error>;
  } = {
    errors: new Map(),
  };

  constructor() {}

  newError(
    source: Source,
    ipaddress: string | undefined = undefined,
    error: any
  ) {
    let text: string;
    if (Object.hasOwn(error, 'error')) {
      text = JSON.stringify(error.error);
    } else if (Object.hasOwn(error, 'message')) {
      text = JSON.stringify(error.message);
    } else {
      text = JSON.stringify(error);
    }

    this.publishError(new Date(), source, ipaddress, text);
  }

  private publishError = (
    date: Date,
    source: Source,
    ipaddress: string | undefined,
    errorMessage: string
  ) => {
    const key = source + '|' + errorMessage;

    var error = this.dataStore.errors.get(key);
    if (!error) {
      error = new Error(source, ipaddress, errorMessage, date, 1);
      this.dataStore.errors.set(key, error);
    } else {
      error.increment();
      error.setLastOccurrance(date);
    }
    this._errors.next(this.dataStore.errors);
  };
}
