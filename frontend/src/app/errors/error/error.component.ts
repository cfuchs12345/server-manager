import { Component, OnInit, OnDestroy } from '@angular/core';
import { ErrorService, Source } from 'src/app/services/errors/error.service';
import { Subscription } from 'rxjs';
import { Error } from 'src/app/services/errors/types';

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
        this.errors = [...errors.values()].sort(
          (error1, error2) =>
            error2.lastOccurrance.getTime() - error1.lastOccurrance.getTime()
        );
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
