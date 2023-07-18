import { Injectable } from '@angular/core';
import { Event, EventType, ObjectType } from './types';
import { isType } from 'src/app/shared/utils';
import { ErrorService, Source } from '../errors/error.service';
import { NGXLogger } from 'ngx-logger';
import {
  Observable,
  Subject,
  Subscription,
  filter,
  of,
  take,
  tap,
} from 'rxjs';
import { ToasterPopupGenerator } from './toaster_messages';
import { Store } from '@ngrx/store';
import { UserToken } from '../users/types';
import { selectAllTokens } from 'src/app/state/usertoken/usertoken.selectors';
import { AuthenticationService } from '../auth/authentication.service';
import { SystemInformation } from '../general/types';

@Injectable({
  providedIn: 'root',
})
export class EventService {
  private eventHandlers: EventHandler<any>[] = []; // eslint-disable-line @typescript-eslint/no-explicit-any

  private _eventSubject = new Subject<[Event, any]>(); // eslint-disable-line @typescript-eslint/no-explicit-any
  private _systemInformationSubject = new Subject<SystemInformation>(); // eslint-disable-line @typescript-eslint/no-explicit-any

  readonly eventSubject$ = this._eventSubject.asObservable();
  readonly systemInformationSubject$ =
    this._systemInformationSubject.asObservable();

  private source: EventSource | undefined;

  private userToken$: Observable<UserToken[]>;

  private map: Map<ObjectType, EventHandlingGetObjectFunction<any>> = new Map(); // eslint-disable-line @typescript-eslint/no-explicit-any

  constructor(
    private store: Store,
    private authService: AuthenticationService,
    private errorService: ErrorService,
    private logger: NGXLogger,
    private toasterMessage: ToasterPopupGenerator
  ) {
    this.userToken$ = this.store.select(selectAllTokens);
    this.userToken$
      .pipe(tap((userTokens) => this.logger.debug('userTokens', userTokens)))
      .subscribe((userTokens) => {
        if (userTokens && userTokens.length > 0) {
          this.authService
            .getEventServiceToken()
            .pipe(
              tap((eventServiceToken) =>
                this.logger.debug('eventServiceToken', eventServiceToken)
              ),
              take(1)
            )
            .subscribe((eventServiceToken) => {
              const event_token = eventServiceToken.token;
              this.source = new EventSource(
                `/backend_nt/events?token=${event_token}`
              );

              this.subscribeToEvents();
              this.subscribeForToasterMessages();

              for (const eventHandler of this.eventHandlers) {
                eventHandler.start();
              }
              this.logger.info('Event connection established');
            });
        } else {
          for (const eventHandler of this.eventHandlers) {
            eventHandler.stop();
          }
          if (this.source) {
            this.source.close();
            this.source = undefined;
          }
          this.logger.info('Event connection closed');
        }
      });
  }

  private subscribeToEvents = () => {
    if (this.source) {
      this.source.addEventListener('message', (message) => {
        const event: Event = JSON.parse(message.data);

        if (isType<Event>(event)) {
          if (event.object_type === 'SystemInformation') {
            const si: SystemInformation = JSON.parse(event.value);
            this._systemInformationSubject.next(si);
          } else {
            this.logger.trace('event received: ', event);

            const object$ =
              event.event_type === 'Delete'
                ? of(undefined)
                : this.getObject(
                    event.object_type,
                    event.key_name,
                    event.key,
                    event.value
                  ); // When we receive a Delete, the object doesn't exist anymore, so we can't select it as object

            object$.pipe(take(1)).subscribe({
              next: (object) => {
                this._eventSubject.next([event, object]);
              },
            });
          }
        }
      });

      this.source.onerror = (e) => {
        this.errorService.newError(Source.EventService, undefined, e);
      };
    }
  };

  subscribeForToasterMessages = () => {
    this.eventSubject$.subscribe((event) => {
      this.toasterMessage.handleEvent(event);
    });
  };

  registerEventHandler = <T>(eventHandler: EventHandler<T>) => {
    this.eventHandlers.push(eventHandler);

    eventHandler.init(this, this.errorService);
  };

  registerGetObjectFunction = (
    objectType: ObjectType,
    func: EventHandlingGetObjectFunction<any> // eslint-disable-line @typescript-eslint/no-explicit-any
  ) => {
    this.map.set(objectType, func);
  };

  getObject = <T>(
    objectType: ObjectType,
    key_name: string,
    key: string,
    value: string
  ): Observable<T> => {
    const func = this.map.get(objectType);

    return func ? func(key_name, key, value, value) : of();
  };
}

export type EventHandlingFunction<T> = (
  eventType: EventType,
  key_name: string,
  key: string,
  value: string,
  object: T
) => void;

export type EventHandlingUpdateFunction<T> = (
  eventType: EventType,
  key_name: string,
  key: string,
  value: string,
  version: number,
  object: T
) => void;

export type EventHandlingGetObjectFunction<T> = (
  key_name: string,
  key: string,
  value: string,
  data: string
) => Observable<T>;

export class EventHandler<T> {
  private eventService: EventService | undefined;
  private errorService: ErrorService | undefined;

  private subscription: Subscription | undefined;

  constructor(
    private objectType: ObjectType,
    private insertFunction: EventHandlingFunction<T>,
    private updateFunction: EventHandlingUpdateFunction<T>,
    private deleteFunction: EventHandlingFunction<T>,
    private getObjectFunction: EventHandlingGetObjectFunction<T>
  ) {}

  init = (eventService: EventService, errorService: ErrorService) => {
    this.eventService = eventService;
    this.errorService = errorService;

    this.eventService.registerGetObjectFunction(
      this.objectType,
      this.getObjectFunction
    );
  };

  start = () => {
    if (!this.eventService) {
      console.log('EventHandler was not initialized before calling start');
      return;
    }

    this.subscription = this.eventService.eventSubject$
      .pipe(
        filter((eventAndObject: [Event, any]) => {
          // eslint-disable-line @typescript-eslint/no-explicit-any
          return eventAndObject[0].object_type === this.objectType;
        })
      )
      .subscribe((eventAndObject: [Event, any]) => {
        // eslint-disable-line @typescript-eslint/no-explicit-any
        const event = eventAndObject[0];
        const currenObject = eventAndObject[1];

        if (event.event_type === 'Insert') {
          if (!this.eventService) {
            return;
          }

          try {
            this.insertFunction(
              event.event_type,
              event.key_name,
              event.key,
              event.value,
              currenObject
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
              event.version,
              currenObject
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
              event.value,
              currenObject
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
