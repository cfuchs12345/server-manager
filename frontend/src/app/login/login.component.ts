import { Component } from '@angular/core';
import { FormControl, FormGroup, Validators } from '@angular/forms';
import { AuthenticationService } from '../services/auth/authentication.service';
import { map, tap, pipe, Observable } from 'rxjs';

import { Router } from '@angular/router';

@Component({
  selector: 'app-login',
  templateUrl: './login.component.html',
  styleUrls: ['./login.component.scss'],
})
export class LoginComponent {
  userIdLabel: string = 'User Id';
  userIdPlaceholder: string = '';
  userIdHint: string = '';
  userId: FormControl = new FormControl('', [
    Validators.required,
    Validators.minLength(4),
  ]);

  passwordLabel: string = 'Password';
  passwordPlaceholder: string = '';
  passwordHint: string = '';
  password: FormControl = new FormControl('', [
    Validators.required,
    Validators.minLength(4),
  ]);

  buttonTextLogin: string = 'Login';

  form = new FormGroup({ userId: this.userId, password: this.password });

  constructor(
    private authService: AuthenticationService,
    private router: Router
  ) {}

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
    const subscription = this.authService.login(this.userId.value, this.password.value).subscribe({
      next: (result) => {
        if (result && result.token) {
          this.router.navigate(['/home']);
        }
      },
      error: (err: any) => {
        this.form.setErrors({
          wrongLogin: 'User Id and/or password is incorrect',
        });
      },
      complete: () => {
        if(subscription) {
          subscription.unsubscribe();
        }
      },
    });
  };
}
