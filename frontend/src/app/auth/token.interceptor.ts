import { HttpInterceptorFn } from '@angular/common/http';
import { inject } from '@angular/core';
import {
  HttpEvent,
} from '@angular/common/http';
import { Observable, switchMap, take } from 'rxjs';
import { Store } from '@ngrx/store';
import { selectToken } from '../state/usertoken/usertoken.selectors';

export const tokenInterceptor: HttpInterceptorFn = (
  request,
  next
): Observable<HttpEvent<any>> => {
  // add header with basic auth credentials if user is logged in and request is to the api url
  const store = inject(Store);

  return store.select(selectToken()).pipe(
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

      return next(request);
    })
  );
};
