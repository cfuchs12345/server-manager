import { Component, OnInit, OnDestroy, LOCALE_ID, Inject } from '@angular/core';
import { ErrorService } from 'src/app/services/errors/error.service';
import { Subscription } from 'rxjs';
import { Error } from 'src/app/services/errors/types';
import {formatDate} from '@angular/common';


@Component({
  selector: 'app-error',
  templateUrl: './error.component.html',
  styleUrls: ['./error.component.scss']
})
export class ErrorComponent  implements OnInit, OnDestroy {
  errors: Error[] = [];
  private subscriptionErrors: Subscription | undefined = undefined;

  constructor(private errorService: ErrorService, @Inject(LOCALE_ID) private locale: string) {}

  ngOnInit(): void {
    this.subscriptionErrors = this.errorService.errors.subscribe((errors) => {
      if (errors) {
        this.errors =  [...errors.values()].sort( (error1, error2) => error2.lastOccurrance.getTime() - error1.lastOccurrance.getTime());
      } else {
        // clear messages when empty message received
        this.errors = [];
      }
    }
    );
  }
  ngOnDestroy(): void {
    if( this.subscriptionErrors ) {
      this.subscriptionErrors.unsubscribe();
    }
  }

  formatDate = (date: Date): string => {
    return formatDate(date, 'yyyy-MM-dd hh:mm:ss', this.locale);
  }
}
