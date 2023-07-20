import { Component, OnDestroy, OnInit, inject } from '@angular/core';
import { SystemInformation } from '../services/general/types';
import { EventService } from '../services/events/event.service';
import { SubscriptionHandler } from '../shared/subscriptionHandler';

@Component({
    selector: 'app-system-information',
    templateUrl: './systeminformation.component.html',
    styleUrls: ['./systeminformation.component.scss'],
    standalone: true,
})
export class SystemInformationComponent implements OnInit, OnDestroy {
  private eventService = inject(EventService);

  private systemInformation: SystemInformation | undefined;

  private subscriptionHandler = new SubscriptionHandler(this);

  ngOnInit() {
    this.subscriptionHandler.subscription =
      this.eventService.systemInformationSubject$.subscribe(
        (systemInformation) => (this.systemInformation = systemInformation)
      );
  }

  ngOnDestroy(): void {
    this.subscriptionHandler.onDestroy();
  }

  find = (
    infoType: 'memory_stats' | 'memory_usage' | 'load_average',
    name: string
  ): number | undefined => {
    if (!this.systemInformation) {
      return undefined;
    }

    switch (infoType) {
      case 'memory_stats': {
        const found = this.systemInformation.memory_stats.find(
          (i) => i.name === name
        );

        return found ? this.round(found.value / 1024 / 1024, 2) : undefined;
      }
      case 'memory_usage': {
        const found = this.systemInformation.memory_usage.find(
          (i) => i.name === name
        );

        return found ? this.round(found.value / 1024 / 1024, 2) : undefined;
      }
      case 'load_average': {
        const found = this.systemInformation.load_average.find(
          (i) => i.name === name
        );

        return found ? this.round(found.value, 4) : undefined;
      }
    }

    return undefined;
  };

  round = (number: number, digits: number): number => {
    return parseFloat(number.toFixed(digits));
  };
}
