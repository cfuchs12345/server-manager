import { Injectable } from '@angular/core';
import {
  HttpRequest,
  HttpHandler,
  HttpEvent,
  HttpInterceptor,
} from '@angular/common/http';
import { Observable, switchMap } from 'rxjs';
import { Store } from '@ngrx/store';
import { selectToken } from '../state/usertoken/usertoken.selectors';
import { UserToken } from '../services/users/types';

@Injectable()
export class TokenInterceptor implements HttpInterceptor {
  userToken$: Observable<UserToken | undefined>;

  constructor(
    private store: Store
  ) {
    this.userToken$ = this.store.select(selectToken());
  }

  /* eslint-disable @typescript-eslint/no-explicit-any */
  intercept(
    request: HttpRequest<any>,
    next: HttpHandler
  ): Observable<HttpEvent<any>> {
    // add header with basic auth credentials if user is logged in and request is to the api url

    return this.userToken$.pipe(
      switchMap((userToken) => {
        const isApiUrl = request.url.startsWith('/backend');

        if (userToken && isApiUrl) {
          request = request.clone({
            setHeaders: {
              Authorization: `Bearer ${userToken.token}`,
            },
          });
        }

        return next.handle(request);
      })
    );
  }
}
