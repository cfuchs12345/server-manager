import { Observable, Subscription, filter } from "rxjs";
import { ErrorService, Source } from "../errors/error.service";
import { EventService } from "./event.service";

export type ObjectType =
  | 'Heartbeat'
  | 'Status'
  | 'Server'
  | 'Plugin'
  | 'DisabledPlugins'
  | 'ConditionCheckResult'
  | 'Notification'
  | 'User'
  | 'SystemInformation';


export type EventType = 'Insert' | 'Update' | 'Delete' | 'Refresh';

export class Event {
  constructor(
    public object_type: ObjectType,
    public event_type: EventType,
    public key_name: string,
    public key: string,
    public value: string,
    public version: number
  ) {}
}

export interface VersionedObject {
  getVersion(): number;
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
        filter((eventAndObject: [Event, any]) => { // eslint-disable-line @typescript-eslint/no-explicit-any
          return eventAndObject[0].object_type === this.objectType;
        })
      )
      .subscribe((eventAndObject: [Event, any]) => { // eslint-disable-line @typescript-eslint/no-explicit-any
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
