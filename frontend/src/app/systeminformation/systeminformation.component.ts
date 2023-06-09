import { Component, OnInit, OnDestroy } from '@angular/core';
import { GeneralService } from '../services/general/general.service';
import { Subscription } from 'rxjs';
import { SystemInformation } from '../services/general/types';

@Component({
  selector: 'app-system-information',
  templateUrl: './systeminformation.component.html',
  styleUrls: ['./systeminformation.component.scss'],
})
export class SystemInformationComponent implements OnInit, OnDestroy {
  private systemInformationSubscription: Subscription | undefined = undefined;
  private systemInformation: SystemInformation | undefined;

  constructor(private generalService: GeneralService) {}

  ngOnInit(): void {
    this.systemInformationSubscription =
      this.generalService.systemInformation.subscribe((info) => {
        this.systemInformation = info;
      });

    this.generalService.getSystemInformation();
  }
  ngOnDestroy(): void {
    if (this.systemInformationSubscription) {
      this.systemInformationSubscription.unsubscribe();
    }
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
