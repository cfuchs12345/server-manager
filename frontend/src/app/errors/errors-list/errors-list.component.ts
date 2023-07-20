import { Component, OnInit, OnDestroy, inject } from '@angular/core';
import { ErrorService } from 'src/app/services/errors/error.service';
import { Subscription } from 'rxjs';
import { ErrorComponent } from '../error/error.component';
import { ExtendedModule } from '@angular/flex-layout/extended';
import { NgClass } from '@angular/common';
import { ConfigurationGroupComponent } from '../../configuration/configuration-group/configuration-group.component';

@Component({
    selector: 'app-errors-list',
    templateUrl: './errors-list.component.html',
    styleUrls: ['./errors-list.component.scss'],
    standalone: true,
    imports: [
        ConfigurationGroupComponent,
        NgClass,
        ExtendedModule,
        ErrorComponent,
    ],
})
export class ErrorsListComponent implements OnInit, OnDestroy {
  private errorService = inject( ErrorService);
  private subscriptionErrors: Subscription | undefined = undefined;

  errorCount = 0;
  classes: string[] = [];


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
