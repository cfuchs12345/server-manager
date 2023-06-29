import { Injectable } from '@angular/core';
import { Event } from './types';
import { isType } from 'src/app/shared/utils';
import { ErrorService, Source } from '../errors/error.service';
import { NGXLogger } from 'ngx-logger';
import { Subject } from 'rxjs';

@Injectable({
  providedIn: 'root',
})
export class EventService {
  private _eventSubject = new Subject<Event>();

  readonly eventSubject = this._eventSubject.asObservable();

  constructor(private errorService: ErrorService, private logger: NGXLogger) {
    this.logger.info("EventService initialized");
    this.subscribeToEvents();
  }

  private subscribeToEvents = () => {
    const source = new EventSource('/backend_nt/events');

    source.addEventListener('message', (message) => {
      let event: Event = JSON.parse(message.data);

      if (isType<Event>(event)) {
        this.logger.info("event received: ", event);
        this._eventSubject.next(event);
      }
    });

    source.onerror = (e) => {
      this.errorService.newError(Source.EventService, undefined, e);
    };
  };
}

