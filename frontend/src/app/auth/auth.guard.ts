import { inject } from '@angular/core';
import {
  ActivatedRouteSnapshot,
  CanActivateFn,
  Router,
  RouterStateSnapshot,
} from '@angular/router';
import { Observable, map, take } from 'rxjs';
import { AuthenticationService } from '../services/auth/authentication.service';

export const AuthGuard: CanActivateFn = (
  next: ActivatedRouteSnapshot,
  state: RouterStateSnapshot
): Observable<boolean> => {
  const router = inject(Router);
  const service = inject(AuthenticationService);

  return service.userToken$.pipe(
    take(1),
    map((userToken) => {
      if (userToken) {
        return true;
      }
      // not logged in so redirect to login page with the return url
      if (state.url && state.url.indexOf('login') < 0) {
        router.navigate(['/login']);
      }
      return false;
    })
  );
};
