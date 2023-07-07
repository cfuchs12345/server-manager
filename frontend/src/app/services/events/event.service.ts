import { Injectable } from '@angular/core';
import { Event, EventType, ObjectType } from './types';
import { isType } from 'src/app/shared/utils';
import { ErrorService, Source } from '../errors/error.service';
import { NGXLogger } from 'ngx-logger';
import { Observable, Subject, Subscription, filter } from 'rxjs';
import { ToasterPopupGenerator } from './toaster_messages';
import { Store } from '@ngrx/store';
import { UserToken } from '../users/types';
import { selectAllTokens } from 'src/app/state/usertoken/usertoken.selectors';

@Injectable({
  providedIn: 'root',
})
export class EventService {
  private eventHandlers: EventHandler[] = [];

  private _eventSubject = new Subject<Event>();

  readonly eventSubject = this._eventSubject.asObservable();

  private source: EventSource | undefined;

  private userToken$: Observable<UserToken[]>;

  constructor(
    private store: Store,
    private errorService: ErrorService,
    private logger: NGXLogger,
    private toasterMessage: ToasterPopupGenerator
  ) {
    this.userToken$ = this.store.select(selectAllTokens);
    this.userToken$.subscribe((tokens) => {

      if (tokens && tokens.length > 0) {
        this.source = new EventSource('/backend_nt/events');

        this.subscribeToEvents();
        this.subscribeForToasterMessages();

        for (const eventHandler of this.eventHandlers) {
          eventHandler.start();
        }
      } else {
        for (const eventHandler of this.eventHandlers) {
          eventHandler.stop();
        }
        if (this.source) {
          this.source.close();
          this.source = undefined;
        }
      }
    });
  }

  private subscribeToEvents = () => {
    if (this.source) {

      this.source.addEventListener('message', (message) => {
        const event: Event = JSON.parse(message.data);

        if (isType<Event>(event)) {
          this.logger.info('event received: ', event);

          this._eventSubject.next(event);
        }
      });

      this.source.onerror = (e) => {
        this.errorService.newError(Source.EventService, undefined, e);
      };
    }
  };

  subscribeForToasterMessages = () => {
    this.eventSubject.subscribe((event) => {
      this.toasterMessage.handleEvent(event);
    });
  };

  registerEventHandler(eventHandler: EventHandler) {
    this.eventHandlers.push(eventHandler);

    eventHandler.init(this, this.errorService);
  }
}

export type EventHandlingFunction = (
  eventType: EventType,
  keyType: string,
  key: string,
  data: string
) => void;

export type EventHandlingUpdateFunction = (
  eventType: EventType,
  keyType: string,
  key: string,
  data: string,
  changeFlag: string
) => void;

export class EventHandler {
  private eventService: EventService | undefined;
  private errorService: ErrorService | undefined;

  private subscription: Subscription | undefined;

  constructor(
    private objectType: ObjectType,
    private insertFunction: EventHandlingFunction,
    private updateFunction: EventHandlingUpdateFunction,
    private deleteFunction: EventHandlingFunction
  ) {}

  init = (eventService: EventService, errorService: ErrorService) => {
    this.eventService = eventService;
    this.errorService = errorService;
  };

  start = () => {
    if (!this.eventService) {
      console.log('EventHandler was not initialized before calling start');
      return;
    }

    this.subscription = this.eventService.eventSubject
      .pipe(
        filter((event: Event) => {
          return event.object_type === this.objectType;
        }),
      )
      .subscribe((event: Event) => {
        if (event.event_type === 'Insert') {
          try {
            this.insertFunction(
              event.event_type,
              event.key_name,
              event.key,
              event.value
            );
          } catch (err) {
            if (this.errorService) {
              this.errorService.newError(
                Source.EventService,
                '',
                new Error(
                  'Insert function failed with error' + JSON.stringify(err)
                )
              );
            }
          }
        } else if (
          event.event_type === 'Update' ||
          event.event_type === 'Refresh'
        ) {
          try {
            this.updateFunction(
              event.event_type,
              event.key_name,
              event.key,
              event.value,
              event.change_flag
            );
          } catch (err) {
            if (this.errorService) {
              this.errorService.newError(
                Source.EventService,
                '',
                new Error(
                  'Update function failed with error' + JSON.stringify(err)
                )
              );
            }
          }
        } else if (event.event_type === 'Delete') {
          try {
            this.deleteFunction(
              event.event_type,
              event.key_name,
              event.key,
              event.value
            );
          } catch (err) {
            if (this.errorService) {
              this.errorService.newError(
                Source.EventService,
                '',
                new Error(
                  'Delete function failed with error' + JSON.stringify(err)
                )
              );
            }
          }
        }
      });
  };

  stop = () => {
    if (this.subscription) {
      this.subscription.unsubscribe();
    }
  };
}
