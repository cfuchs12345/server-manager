import { Injectable } from '@angular/core';
import {
  HttpRequest,
  HttpHandler,
  HttpEvent,
  HttpInterceptor,
} from '@angular/common/http';
import { Observable, throwError } from 'rxjs';
import { catchError } from 'rxjs/operators';
import { AuthenticationService } from '../services/auth/authentication.service';

@Injectable()
export class ErrorInterceptor implements HttpInterceptor {
  constructor(private authenticationService: AuthenticationService) {}

  intercept(
    // eslint-disable-next-line  @typescript-eslint/no-explicit-any
    request: HttpRequest<any>,
    // eslint-disable-next-line  @typescript-eslint/no-explicit-any
    next: HttpHandler
  ): Observable<HttpEvent<any>> {// eslint-disable-line  @typescript-eslint/no-explicit-any
    return next.handle(request).pipe(
      catchError((err) => {
        if (err.status === 401) {
          // auto logout if 401 response returned from api
          this.authenticationService.logout();
        }
        return throwError(() => err);
      })
    );
  }
}
