import { Component, OnInit, OnDestroy } from '@angular/core';
import { ErrorService } from 'src/app/services/errors/error.service';
import { Subscription } from 'rxjs';

@Component({
  selector: 'app-errors-list',
  templateUrl: './errors-list.component.html',
  styleUrls: ['./errors-list.component.scss'],
})
export class ErrorsListComponent implements OnInit, OnDestroy {
  errorCount = 0;
  classes: string[] = [];

  private subscriptionErrors: Subscription | undefined = undefined;

  constructor(private errorService: ErrorService) {}

  ngOnInit(): void {
    this.subscriptionErrors = this.errorService.errors.subscribe(() => {
      this.errorCount = this.errorCount + 1;
    });
  }
  ngOnDestroy(): void {
    if (this.subscriptionErrors) {
      this.subscriptionErrors.unsubscribe();
    }
  }

  flashErrorList = () => {
    this.classes.push('flash');
    setTimeout(() => {
      this.classes.splice(0, this.classes.length);
    }, 500);
  };
}
