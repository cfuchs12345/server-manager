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
  const authService = inject(AuthenticationService);


  const userToken = authService.getUserToken();

  if (userToken) {
    return true;
  }
  // not logged in so redirect to login page with the return url
  if (state.url && state.url.indexOf('login') < 0) {
    router.navigate(['/login']);
  }
  return false;
};
