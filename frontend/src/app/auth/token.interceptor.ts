import { Injectable, inject } from '@angular/core';
import {
  HttpRequest,
  HttpHandler,
  HttpEvent,
  HttpInterceptor,
} from '@angular/common/http';
import { Observable, switchMap, take } from 'rxjs';
import { Store } from '@ngrx/store';
import { selectToken } from '../state/usertoken/usertoken.selectors';

@Injectable()
export class TokenInterceptor implements HttpInterceptor {
  private store = inject(Store);

  /* eslint-disable @typescript-eslint/no-explicit-any */
  intercept(
    request: HttpRequest<any>,
    next: HttpHandler
  ): Observable<HttpEvent<any>> {
    // add header with basic auth credentials if user is logged in and request is to the api url

    return this.store.select(selectToken()).pipe(
      take(1),
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
