import { OnDestroy } from "@angular/core";

interface Unsubscribable {
  unsubscribe(): void;
}

export class SubscriptionHandler {
  private unsubscribables:  Unsubscribable[] = [];

  constructor(private component: OnDestroy){

  }

  onDestroy() {
    for( const unsubscribable of this.unsubscribables ) {
      unsubscribable.unsubscribe();
    }
  }

  set subscription( unsubscribable: Unsubscribable) {
    this.unsubscribables.push(unsubscribable);
  }
}
