import { OnDestroy } from '@angular/core';
import { NGXLogger } from 'ngx-logger';

interface Unsubscribable {
  unsubscribe(): void;
}

export class SubscriptionHandler {
  private unsubscribables: Unsubscribable[] = [];

  constructor(private component: OnDestroy) {}

  set subscription(unsubscribable: Unsubscribable) {
    this.unsubscribables.push(unsubscribable);
  }

  onDestroy(logger: NGXLogger | undefined = undefined) {
    const count = this.unsubscribables.length;

    this.unsubscribe();
    this.clearSubscribables();

    this.log(logger, count);
  }
  private unsubscribe = () => {
    for (const unsubscribable of this.unsubscribables) {
      unsubscribable.unsubscribe();
    }
  };
  private clearSubscribables = () => {
    this.unsubscribables.splice(0, this.unsubscribables.length);
  };

  private log = (logger: NGXLogger | undefined, count: number) => {
    const msg =
      'Unsubscribed all tracked subscriptions (count: ' +
      count +
      ') for ' +
      this.component.constructor.name;
    logger ? logger.debug(msg) : console.log(msg);
  };
}
