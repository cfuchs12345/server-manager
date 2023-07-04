import { Observable, tap, map } from 'rxjs';
import { inject } from '@angular/core';
import {
  ActivatedRouteSnapshot,
  CanActivateFn,
  Router,
  RouterStateSnapshot,
} from '@angular/router';
import { AuthenticationService } from '../services/auth/authentication.service';

export const RegisterGuard: CanActivateFn = (
  next: ActivatedRouteSnapshot,
  state: RouterStateSnapshot
): Observable<boolean> => {
  const router = inject(Router);
  const service = inject(AuthenticationService);

  return service.userExist().pipe(
    map((exist) => (exist !== undefined ? exist : false)),
    tap((exist) => {
      if (exist) {
        if (!state.url || state.url.indexOf('login') < 0) {
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
