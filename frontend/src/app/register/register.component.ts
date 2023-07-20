import { Component, OnDestroy, inject } from '@angular/core';
import { FormControl, Validators, FormsModule, ReactiveFormsModule } from '@angular/forms';

import { Router } from '@angular/router';
import { UserService } from '../services/users/users.service';
import { User } from '../services/users/types';
import { MatDialog } from '@angular/material/dialog';
import { MessageDialogComponent } from '../ui/message_dialog/message-dialog.component';
import { SubscriptionHandler } from '../shared/subscriptionHandler';
import { MatButtonModule } from '@angular/material/button';
import { NgIf } from '@angular/common';
import { MatInputModule } from '@angular/material/input';
import { MatFormFieldModule } from '@angular/material/form-field';
import { FlexModule } from '@angular/flex-layout/flex';

@Component({
    selector: 'app-register',
    templateUrl: './register.component.html',
    styleUrls: ['./register.component.scss'],
    standalone: true,
    imports: [
        FlexModule,
        MatFormFieldModule,
        MatInputModule,
        FormsModule,
        ReactiveFormsModule,
        NgIf,
        MatButtonModule,
    ],
})
export class RegisterComponent implements OnDestroy {
  private userService = inject(UserService);
  private router = inject(Router);
  private dialog = inject(MatDialog);

  userIdLabel = 'User Id';
  useridPlaceholder = '';
  userIdHint = '';

  userId = new FormControl('', [Validators.required, Validators.minLength(4)]);

  fullNameLabel = 'Full Name';
  fullNamePlaceholder = '';
  fullNameHint = '';

  fullName = new FormControl('', [
    Validators.required,
    Validators.minLength(4),
  ]);

  emailLabel = 'E-Mail';
  emailPlaceholder = '';
  emailHint = 'An initial password will be send to this address';

  email = new FormControl('', [Validators.required, Validators.email]);

  buttonText = 'Save';

  subscriptionHandler = new SubscriptionHandler(this);

  ngOnDestroy(): void {
    this.subscriptionHandler.onDestroy();
  }

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
      this.subscriptionHandler.subscription = this.userService
        .saveUser(
          new User(this.userId.value, this.fullName.value, this.email.value),
          true
        )
        .subscribe((passwd) => {
          if (passwd) {
            if (passwd.password) {
              this.subscriptionHandler.subscription = this.dialog
                .open(MessageDialogComponent, {
                  data: {
                    title: 'Initial Password',
                    message:
                      'The initial password for user ' +
                      passwd.user_id +
                      ' is: "' +
                      passwd.password +
                      '"',
                  },
                })
                .afterClosed()
                .subscribe(() => {
                  setTimeout(() => {
                    this.router.navigate(['/login']);
                  }, 50);
                });
            } else {
              setTimeout(() => {
                this.router.navigate(['/login']);
              }, 50);
            }
          }
        });
    }
  };
}
