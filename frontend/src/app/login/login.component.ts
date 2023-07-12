import { Component, OnDestroy } from '@angular/core';
import { FormControl, FormGroup, Validators } from '@angular/forms';
import { AuthenticationService } from '../services/auth/authentication.service';
import { Router } from '@angular/router';
import { Store } from '@ngrx/store';
import * as GlobalActions from '../state/global.actions';
import { SubscriptionHandler } from '../shared/subscriptionHandler';
import { take } from 'rxjs';

@Component({
  selector: 'app-login',
  templateUrl: './login.component.html',
  styleUrls: ['./login.component.scss'],
})
export class LoginComponent implements OnDestroy {
  userIdLabel = 'User Id';
  userIdPlaceholder = '';
  userIdHint = '';
  userId: FormControl = new FormControl('', [
    Validators.required,
    Validators.minLength(4),
  ]);

  passwordLabel = 'Password';
  passwordPlaceholder = '';
  passwordHint = '';
  password: FormControl = new FormControl('', [
    Validators.required,
    Validators.minLength(4),
  ]);

  buttonTextLogin = 'Login';

  form = new FormGroup({ userId: this.userId, password: this.password });

  subscriptionHandler = new SubscriptionHandler(this);

  constructor(
    private store: Store,
    private authService: AuthenticationService,
    private router: Router
  ) {}

 ngOnDestroy(): void {
   this.subscriptionHandler.onDestroy();
 }

  getErrorMessagUserId = (): string => {
    if (this.userId.hasError('required')) {
      return 'You need to enter the user Id';
    } else if (this.userId.hasError('minlength')) {
      return 'The user Id has to be at least 4 chars long';
    } else if (this.userId.invalid) {
      return 'Unknown validation error';
    }
    return '';
  };

  getErrorMessagPassword = (): string => {
    if (this.password.hasError('required')) {
      return 'You need to enter the password';
    }
    if (this.password.hasError('minlength')) {
      return 'The password has to be at least 6 chars long';
    } else if (this.password.invalid) {
      return 'Unknown validation error';
    }
    return '';
  };

  getLoginMessage = (): string => {
    if (this.form.hasError('wrongLogin')) {
      return this.form.getError('wrongLogin');
    }
    return '';
  };

  onClickLogin = () => {
    // even if no TLS/HTTPS is used, we don't want to transfer a cleartext password
    // so we use a encryption here and the server is then checking the password against the hash value on the server side
    this.subscriptionHandler.subscription = this.authService
      .login(this.userId.value, this.password.value)
      .pipe(take(1))
      .subscribe({
        next: (userToken) => {
          this.store.dispatch(GlobalActions.init({userToken: userToken}));

          if (userToken && userToken.token) {
            this.router.navigate(['/home']);
          }
        },
        error: () => {
          this.form.setErrors({
            wrongLogin: 'User Id and/or password is incorrect',
          });
        },
      });
  };
}
