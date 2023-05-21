import { Component } from '@angular/core';
import { FormControl } from '@angular/forms';
import { AuthenticationService } from '../services/auth/authentication.service';
import { map, tap, pipe, Observable } from 'rxjs';

import { Router } from '@angular/router';

@Component({
  selector: 'app-login',
  templateUrl: './login.component.html',
  styleUrls: ['./login.component.scss']
})
export class LoginComponent {
  userIdLabel: string = 'User Id';
  userIdPlaceholder: string = '';
  userIdHint: string = '';
  userId : FormControl = new FormControl();

  passwordLabel: string = 'Password';
  passwordPlaceholder: string = '';
  passwordHint: string = '';
  password : FormControl = new FormControl();

  buttonTextLogin: string = 'Login';

  constructor(private authService: AuthenticationService, private router: Router) {}

  getErrorMessagUserId = (): string => {
    return "";
  }

  getErrorMessagPassword = (): string => {
    return "";
  }

  onClickLogin = () => {
    // even if no TLS/HTTPS is used, we don't want to transfer a cleatext password
    // so we use a encryption here and the server is then checking the password against the hash value on the server side
    this.authService.login(this.userId.value, this.password.value).subscribe(
      (result) => {
        if ( result && result.token) {
          this.router.navigate(['/home']);
        }
      }
    );
  }
}
