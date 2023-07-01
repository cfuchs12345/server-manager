import { Observable, tap, of } from 'rxjs';
import { inject } from '@angular/core';
import {
  ActivatedRouteSnapshot,
  CanActivateFn,
  Router,
  RouterStateSnapshot,
} from '@angular/router';
import { AuthenticationService } from '../services/auth/authentication.service';

let usersExist = false;

export const RegisterGuard: CanActivateFn = (
  next: ActivatedRouteSnapshot,
  state: RouterStateSnapshot
): Observable<boolean> => {
  const router = inject(Router);
  const service = inject(AuthenticationService);

  if (usersExist) {
    if (!state.url || state.url.indexOf('login') < 0) {
      router.navigate(['/login']);
    }
    return of(true);
  }

  return service.userExist().pipe(
    tap((res) => {
      if (res) {
        usersExist = true;
        if (!state.url  || state.url.indexOf('login') < 0) {
          router.navigate(['/login']);
        }
      } else {
        if (!state.url || state.url.indexOf('register') < 0) {
          router.navigate(['/register']);
        }
      }
    })
  );
};
