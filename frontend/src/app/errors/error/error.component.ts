import { Component, OnInit, OnDestroy } from '@angular/core';
import { ErrorService } from 'src/app/services/errors/error.service';
import { Subscription } from 'rxjs';
import { Error } from 'src/app/services/errors/types';
import { mapValuesToArray, sortByNumericField } from 'src/app/shared/utils';

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
    this.subscriptionErrors = this.errorService.errors.subscribe((errors) => {
      if (errors) {
        this.errors = sortByNumericField(mapValuesToArray(errors), (error) => error.lastOccurrance.getTime() );
      } else {
        // clear messages when empty message received
        this.errors = [];
      }
    });
  }
  ngOnDestroy(): void {
    if (this.subscriptionErrors) {
      this.subscriptionErrors.unsubscribe();
    }
  }
}
