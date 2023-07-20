import { Component, OnDestroy, OnInit, inject } from '@angular/core';
import { AbstractControl, FormBuilder, FormControl, FormGroup, Validators, FormsModule, ReactiveFormsModule } from '@angular/forms';
import { Store } from '@ngrx/store';
import { Observable } from 'rxjs';
import { EncryptionService } from 'src/app/services/encryption/encryption.service';
import { UserToken } from 'src/app/services/users/types';
import { UserService } from 'src/app/services/users/users.service';
import { selectToken } from 'src/app/state/usertoken/usertoken.selectors';
import { SubscriptionHandler } from 'src/app/shared/subscriptionHandler';
import { MatButtonModule } from '@angular/material/button';
import { NgIf } from '@angular/common';
import { MatInputModule } from '@angular/material/input';
import { MatFormFieldModule } from '@angular/material/form-field';
import { FlexModule } from '@angular/flex-layout/flex';

@Component({
    selector: 'app-change-password-modal',
    templateUrl: './change-password-modal.component.html',
    styleUrls: ['./change-password-modal.component.scss'],
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
export class ChangePasswordModalComponent implements OnDestroy, OnInit {
  private store = inject(Store);
  private userService = inject(UserService);
  private encryptionService = inject(EncryptionService);
  private formBuilder = inject(FormBuilder);

  private userToken$?: Observable<UserToken | undefined>;

  private subscriptionHandler = new SubscriptionHandler(this);

  private form: FormGroup = new FormGroup({});

  buttonText = 'Change the password';

  oldPasswordLabel = 'Old Password';
  oldPasswordPlaceholder = '';
  oldPasswordHint = 'Please enter your current password';
  oldPassword = new FormControl('', [
    Validators.required,
    Validators.minLength(6),
  ]);

  newPasswordLabel = 'New Password';
  newPasswordPlaceholder = '';
  newPasswordHint = 'Please enter your new password';
  newPassword = new FormControl('', [
    Validators.required,
    Validators.minLength(6),
  ]);

  confirmNewPasswordLabel = 'Confirm new Password';
  confirmNewPasswordPlaceholder = '';
  confirmNewPasswordHint =
    'Please enter your new password again to avoid typos';
  confirmNewPassword = new FormControl('', [
    Validators.required,
    Validators.minLength(6),
  ]);



  ngOnInit(): void {
    this.userToken$ = this.store.select(selectToken());

    this.form = this.formBuilder.group({
      oldPassword: this.oldPassword,
      newPassword: this.newPassword,
      confirmNewPassword: this.confirmNewPassword,
    });

    this.form.addValidators(
      this.mustMatch('newPassword', 'confirmNewPassword')
    );
  }

  ngOnDestroy(): void {
    this.subscriptionHandler.onDestroy();
  }

  onClickChangePassword = () => {
    this.subscriptionHandler.subscription = this.encryptionService
      .requestOneTimeKey()
      .subscribe({
        next: (otk) => {
          if (!this.userToken$) {
            return;
          }

          this.subscriptionHandler.subscription = this.userToken$
            .pipe()
            .subscribe((token) => {
              if (
                this.oldPassword.valid &&
                this.oldPassword.value &&
                this.newPassword.valid &&
                this.newPassword.value &&
                this.confirmNewPassword.valid &&
                token
              ) {
                this.userService.changePassword(
                  token.user_id,
                  this.oldPassword.value,
                  this.newPassword.value,
                  otk
                );
              }
            });
        },
      });
  };

  getOldPasswordMessage = (): string => {
    if (this.oldPassword.hasError('required')) {
      return 'You must enter the old password';
    }
    return this.oldPassword.hasError('minlength')
      ? 'The password is not long enough (at least 6 chars)'
      : 'Unknown error';
  };

  getNewPasswordMessage = (): string => {
    if (this.newPassword.hasError('required')) {
      return 'You must enter the new password';
    }
    return this.newPassword.hasError('minlength')
      ? 'The password is not long enough (at least 6 chars)'
      : 'Unknown error';
  };

  getConfirmNewPasswordMessage = (): string => {
    if (this.confirmNewPassword.hasError('required')) {
      return 'You must re-enter the new password';
    }
    if (this.confirmNewPassword.hasError('mustMatch')) {
      return 'The passwords do not match';
    }
    return this.confirmNewPassword.hasError('minlength')
      ? 'The password is not long enough (at least 6 chars)'
      : 'Unknown error';
  };

  mustMatch = (controlName: string, matchingControlName: string) => {
    return (group: AbstractControl) => {
      const control = group.get(controlName);
      const matchingControl = group.get(matchingControlName);

      if (!control || !matchingControl) {
        return null;
      }

      // return if another validator has already found an error on the matchingControl
      if (matchingControl.errors) {
        return null;
      }

      // set error on matchingControl if validation fails
      if (control.value !== matchingControl.value) {
        matchingControl.setErrors({ mustMatch: true });
      } else {
        matchingControl.setErrors(null);
      }
      return null;
    };
  };
}
