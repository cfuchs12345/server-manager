import { Component, OnInit, OnDestroy } from '@angular/core';
import { ErrorService } from 'src/app/services/errors/error.service';
import { Subscription } from 'rxjs';

@Component({
  selector: 'app-errors-list',
  templateUrl: './errors-list.component.html',
  styleUrls: ['./errors-list.component.scss']
})
export class ErrorsListComponent  implements OnInit, OnDestroy  {
  errorCount: number = 0;
  classes: string[] = [];


  private subscriptionErrors: Subscription | undefined = undefined;

  constructor(private errorService: ErrorService) {}

  ngOnInit(): void {
    this.subscriptionErrors = this.errorService.errors.subscribe((errors) => {
      if (errors) {

        var count = 0;
        for( let [i, error] of errors.entries()) {
          count += error.count;
        }
        if( this.errorCount < count  ) {
          setTimeout(this.flashErrorList, 0);
        }
        this.errorCount = count;
      } else {
        // clear messages when empty message received
        this.errorCount = 0;
      }
    }
    );
  }
  ngOnDestroy(): void {
    if( this.subscriptionErrors ) {
      this.subscriptionErrors.unsubscribe();
    }
  }

  flashErrorList = () => {
    this.classes.push("flash");
    setTimeout( () => {
      this.classes.splice(0, this.classes.length);
    }, 500);
  }

}
