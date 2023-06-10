import { Component, OnInit } from '@angular/core';
import {
  AbstractControl,
  FormBuilder,
  FormControl,
  FormGroup,
  Validators,
} from '@angular/forms';
import { AuthenticationService } from 'src/app/services/auth/authentication.service';
import { EncryptionService } from 'src/app/services/encryption/encryption.service';
import { UserService } from 'src/app/services/users/users.service';

@Component({
  selector: 'app-change-password-modal',
  templateUrl: './change-password-modal.component.html',
  styleUrls: ['./change-password-modal.component.scss'],
})
export class ChangePasswordModalComponent implements OnInit {
  buttonText: string = 'Change the password';

  oldPasswordLabel: string = 'Old Password';
  oldPasswordPlaceholder: string = '';
  oldPasswordHint: string = 'Please enter your current password';
  oldPassword = new FormControl('', [
    Validators.required,
    Validators.minLength(6),
  ]);

  newPasswordLabel: string = 'New Password';
  newPasswordPlaceholder: string = '';
  newPasswordHint: string = 'Please enter your new password';
  newPassword = new FormControl('', [
    Validators.required,
    Validators.minLength(6),
  ]);

  confirmNewPasswordLabel: string = 'Confirm new Password';
  confirmNewPasswordPlaceholder: string = '';
  confirmNewPasswordHint: string =
    'Please enter your new password again to avoid typos';
  confirmNewPassword = new FormControl('', [
    Validators.required,
    Validators.minLength(6),
  ]);

  form: FormGroup = new FormGroup({});

  constructor(
    private userService: UserService,
    private authService: AuthenticationService,
    private encryptionService: EncryptionService,
    private formBuilder: FormBuilder
  ) {
    this.form = this.formBuilder.group({
      oldPassword: this.oldPassword,
      newPassword: this.newPassword,
      confirmNewPassword: this.confirmNewPassword,
    });

    this.form.addValidators(
      this.mustMatch('newPassword', 'confirmNewPassword')
    );
  }

  ngOnInit(): void {}

  onClickChangePassword = () => {
    const otk = this.encryptionService.requestOneTimeKey().subscribe((otk) => {
      if (
        this.oldPassword.valid &&
        this.oldPassword.value &&
        this.newPassword.valid &&
        this.newPassword.value &&
        this.confirmNewPassword.valid &&
        this.authService.userToken
      ) {
        this.userService.changePassword(
          this.authService.userToken.user_id,
          this.oldPassword.value,
          this.newPassword.value,
          otk
        );
      }
    });
  };

  getOldPasswordMessage = (): string => {
    if (this.oldPassword.hasError('required')) {
      return 'You must enter the old password';
    }
    return this.oldPassword.hasError('minlength')
      ? 'The password is not long enough (at least 6 chars)'
      : 'Unknown error';
  }

  getNewPasswordMessage = (): string => {
    if (this.newPassword.hasError('required')) {
      return 'You must enter the new password';
    }
    return this.newPassword.hasError('minlength')
      ? 'The password is not long enough (at least 6 chars)'
      : 'Unknown error';
  }

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
  }

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
