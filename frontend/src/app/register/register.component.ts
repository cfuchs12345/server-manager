import { Component } from '@angular/core';
import { FormControl, Validators } from '@angular/forms';
import { Subscription } from 'rxjs';

import { Router } from '@angular/router';
import { UserService } from '../services/users/users.service';
import { User } from '../services/users/types';
import { MatDialog } from '@angular/material/dialog';
import { MessageDialogComponent } from '../ui/message_dialog/message-dialog.component';

@Component({
  selector: 'app-register',
  templateUrl: './register.component.html',
  styleUrls: ['./register.component.scss'],
})
export class RegisterComponent  {
  userIdLabel: string = 'User Id';
  useridPlaceholder: string = '';
  userIdHint: string = '';

  userId = new FormControl('', [Validators.required, Validators.minLength(4)]);

  fullNameLabel: string = 'Full Name';
  fullNamePlaceholder: string = '';
  fullNameHint: string = '';

  fullName = new FormControl('', [
    Validators.required,
    Validators.minLength(4),
  ]);

  emailLabel: string = 'E-Mail';
  emailPlaceholder: string = '';
  emailHint: string = 'An initial password will be send to this address';

  email = new FormControl('', [Validators.required, Validators.email]);

  buttonText: string = 'Save';

  initialPasswordSubscription: Subscription | undefined = undefined;

  constructor(
    private userService: UserService,
    private router: Router,
    private dialog: MatDialog
  ) {}


  getPasswordMessage = (): string => {
    return '';
  };

  getUserIdMessage = (): string => {
    return 'You need to enter a user id with at least 4 characters';
  };

  getFullNameMessage = (): string => {
    return '';
  };

  getEmailMessage = (): string => {
    return 'The given email address is invalid';
  };

  onClickSave = () => {
    if (
      this.userId !== null &&
      this.userId.value !== null &&
      this.fullName !== null &&
      this.fullName.value !== null &&
      this.email !== null &&
      this.email.value !== null
    ) {
      this.initialPasswordSubscription = this.userService.saveUser(
        new User(this.userId.value, this.fullName.value, this.email.value),
        true
      )
        .subscribe({
          next: (userInitialPassword) => {
            if (userInitialPassword.password) {
              this.dialog
                .open(MessageDialogComponent, {
                  data: {
                    title: 'Initial Password',
                    message:
                      'The initial password for user ' +
                      userInitialPassword.user_id +
                      ' is: "' +
                      userInitialPassword.password +
                      '"',
                  },
                })
                .afterClosed()
                .subscribe((any) => {
                  setTimeout(() => {
                    this.router.navigate(['/login']);
                  }, 50);
                });
            } else {
              setTimeout(() => {
                this.router.navigate(['/login']);
              }, 50);
            }
          },
          complete: () => {
            this.initialPasswordSubscription?.unsubscribe();
          }
          });


    }
  };
}
