import { Component, OnDestroy, inject } from '@angular/core';
import { FormControl, FormGroup, Validators, FormsModule, ReactiveFormsModule } from '@angular/forms';
import { AuthenticationService } from '../services/auth/authentication.service';
import { Router } from '@angular/router';
import { Store } from '@ngrx/store';
import * as GlobalActions from '../state/global.actions';
import { SubscriptionHandler } from '../shared/subscriptionHandler';
import { take } from 'rxjs';
import { MatButtonModule } from '@angular/material/button';
import { NgIf } from '@angular/common';
import { MatInputModule } from '@angular/material/input';
import { MatFormFieldModule } from '@angular/material/form-field';
import { FlexModule } from '@angular/flex-layout/flex';

@Component({
    selector: 'app-login',
    templateUrl: './login.component.html',
    styleUrls: ['./login.component.scss'],
    standalone: true,
    imports: [
        FormsModule,
        ReactiveFormsModule,
        FlexModule,
        MatFormFieldModule,
        MatInputModule,
        NgIf,
        MatButtonModule,
    ],
})
export class LoginComponent implements OnDestroy {
  private store = inject(Store);
  private authService = inject(AuthenticationService);
  private router = inject(Router);

  private subscriptionHandler = new SubscriptionHandler(this);

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
          this.store.dispatch(GlobalActions.init({ userToken: userToken }));

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
