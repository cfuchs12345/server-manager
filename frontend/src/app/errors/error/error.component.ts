import { Component, OnInit, OnDestroy } from '@angular/core';
import { ErrorService } from 'src/app/services/errors/error.service';
import { Subscription } from 'rxjs';
import { Error } from 'src/app/services/errors/types';
import { sortByNumericField } from 'src/app/shared/utils';

@Component({
  selector: 'app-error',
  templateUrl: './error.component.html',
  styleUrls: ['./error.component.scss'],
})
export class ErrorComponent implements OnInit, OnDestroy {
  errors: Error[] = [];
  private subscriptionErrors: Subscription | undefined = undefined;

  constructor(private errorService: ErrorService) {}

  ngOnInit(): void {
    this.subscriptionErrors = this.errorService.errors.subscribe((error) => {
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
    });
  }
  ngOnDestroy(): void {
    if (this.subscriptionErrors) {
      this.subscriptionErrors.unsubscribe();
    }
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
