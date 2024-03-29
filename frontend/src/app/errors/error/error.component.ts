import { Component, OnDestroy, OnInit, inject } from '@angular/core';
import { ErrorService } from 'src/app/services/errors/error.service';
import { Error } from 'src/app/services/errors/types';
import { sortByNumericField } from 'src/app/shared/utils';
import { SubscriptionHandler } from 'src/app/shared/subscriptionHandler';
import { ErrorSourceNamePipe } from '../../shared/error-enum-name.pipe';
import { NgFor, DatePipe } from '@angular/common';

@Component({
    selector: 'app-error',
    templateUrl: './error.component.html',
    styleUrls: ['./error.component.scss'],
    standalone: true,
    imports: [
        NgFor,
        DatePipe,
        ErrorSourceNamePipe,
    ],
})
export class ErrorComponent implements OnInit, OnDestroy {
  private errorService = inject(ErrorService);

  private subscriptionHandler = new SubscriptionHandler(this);

  errors: Error[] = [];

  ngOnInit() {
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
