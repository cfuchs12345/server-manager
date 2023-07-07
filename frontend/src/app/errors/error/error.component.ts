import { Component,  OnDestroy } from '@angular/core';
import { ErrorService } from 'src/app/services/errors/error.service';
import { Error } from 'src/app/services/errors/types';
import { sortByNumericField } from 'src/app/shared/utils';
import { SubscriptionHandler } from 'src/app/shared/subscriptionHandler';

@Component({
  selector: 'app-error',
  templateUrl: './error.component.html',
  styleUrls: ['./error.component.scss'],
})
export class ErrorComponent implements OnDestroy {
  errors: Error[] = [];
  private subscriptionHandler = new SubscriptionHandler(this);

  constructor(private errorService: ErrorService) {
    this.subscriptionHandler.subscription = this.errorService.errors.subscribe(
      (error) => {
        const found = this.errors.find(
          (existing) => this.key(existing) === this.key(error)
        );

        if (found) {
          found.setLastOccurrance(error.lastOccurrance);
          found.increment();
        } else {
          this.errors.push(error);
        }
        this.sort();
      }
    );
  }

  ngOnDestroy(): void {
    this.subscriptionHandler.onDestroy();
  }

  sort = () => {
    if (this.errors) {
      this.errors = sortByNumericField(this.errors, (error) =>
        error.lastOccurrance.getTime()
      );
    }
  };

  key = (error: Error): string => {
    return error.source + '|' + error.errorMessage;
  };
}
