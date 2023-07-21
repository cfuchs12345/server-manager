import { HttpInterceptorFn } from '@angular/common/http';
import {  inject } from '@angular/core';
import {
  HttpEvent,
} from '@angular/common/http';
import { Observable, throwError } from 'rxjs';
import { catchError } from 'rxjs/operators';
import { AuthenticationService } from '../services/auth/authentication.service';


export const errorInterceptor: HttpInterceptorFn = (
  request,
  next
): Observable<HttpEvent<any>> => { // eslint-disable-line  @typescript-eslint/no-explicit-any
  const authenticationService = inject( AuthenticationService);

  return next(request).pipe(
    catchError((err) => {
      if (err.status === 401) {
        // auto logout if 401 response returned from api
        authenticationService.logout();
      }
      return throwError(() => err);
    })
  );
}
