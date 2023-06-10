import { inject } from '@angular/core';
import {
  ActivatedRouteSnapshot,
  CanActivateFn,
  Router,
  RouterStateSnapshot,
} from '@angular/router';
import { AuthenticationService } from '../services/auth/authentication.service';

export const AuthGuard: CanActivateFn = (
  next: ActivatedRouteSnapshot,
  state: RouterStateSnapshot
): boolean => {
  const router = inject(Router);
  const service = inject(AuthenticationService);

  if (
    service.userToken !== undefined &&
    service.userToken.token !== undefined
  ) {
    return true;
  }
  // not logged in so redirect to login page with the return url
  if (state.url === undefined || state.url.indexOf('login') < 0) {
    router.navigate(['/login']);
  }
  return false;
};
