import { Component, OnDestroy } from '@angular/core';
import { FormControl, Validators } from '@angular/forms';
import { MatDialog } from '@angular/material/dialog';
import { Store } from '@ngrx/store';
import { Subscription, Observable } from 'rxjs';
import { User } from 'src/app/services/users/types';
import { UserService } from 'src/app/services/users/users.service';
import { ConfirmDialogComponent } from 'src/app/ui/confirm-dialog/confirm-dialog.component';
import { MessageDialogComponent } from 'src/app/ui/message_dialog/message-dialog.component';
import { selectAllUsers } from 'src/app/state/user/user.selectors';

@Component({
  selector: 'app-configure-users-modal',
  templateUrl: './configure-users-modal.component.html',
  styleUrls: ['./configure-users-modal.component.scss'],
})
export class ConfigureUsersModalComponent implements OnDestroy {
  buttonTextAdd = 'Add User';
  buttonTextDelete = 'Delete User';

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

  displayedColumns = ['delete', 'user_id', 'full_name'];

  users$: Observable<User[]>;
  users: User[] = [];

  selectedUsers: string[] = [];
  usersSubscription: Subscription | undefined = undefined;
  initialPasswordSubscription: Subscription | undefined = undefined;

  selectAll = false;

  constructor(private store: Store, private userService: UserService, private dialog: MatDialog) {
    this.users$ = this.store.select(selectAllUsers);
    this.users$.subscribe( (users) => this.users = users);

    this.userService.listUsers();
  }

  ngOnDestroy(): void {
    if (this.usersSubscription) {
      this.usersSubscription.unsubscribe();
    }
  }

  onClickSaveUser = () => {
    if (
      this.userId.value !== null &&
      this.fullName.value !== null &&
      this.email.value !== null
    ) {
      this.userService
        .saveUser(
          new User(this.userId.value, this.fullName.value, this.email.value),
          false
        )
        .subscribe((response) => {
          if (response) {
            if (response.password !== null) {
              this.dialog.open(MessageDialogComponent, {
                data: {
                  title: 'Initial Password',
                  message:
                    'The initial password for user ' +
                    response.user_id +
                    ' is: "' +
                    response.password +
                    '"',
                },
              });
            }
            else {
              // sent by mail
            }
          }
        });
    }
  };

  onClickDeleteUsers = () => {
    const usersToDelete = this.users.filter((user) =>
      this.selectedUsers.find((str) => user.user_id === str)
    );
    const message =
      usersToDelete.length > 1
        ? 'Do you really want to delete ' + usersToDelete.length + ' users?'
        : 'Do you really want to delete the user: ' +
          usersToDelete[0].user_id +
          '?';
    const confirmDialog = this.dialog.open(ConfirmDialogComponent, {
      data: {
        title: 'Confirm User Deletion',
        message,
      },
    });
    confirmDialog.afterClosed().subscribe((result) => {
      if (result === true) {
        this.userService.deleteUsers(usersToDelete);
        this.selectedUsers = [];
        this.selectAll = false;
      }
    });
  };

  isSelected = (user: User): boolean => {
    return this.selectedUsers.indexOf(user.user_id) >= 0;
  };

  usersSelected = (): number => {
    return this.selectedUsers.length;
  };

  onClickSelectUser = (user: User) => {
    if (this.selectedUsers && this.selectedUsers.indexOf(user.user_id) < 0) {
      this.selectedUsers.push(user.user_id);
    } else {
      this.selectedUsers = this.selectedUsers.filter(
        (user_id) => user_id !== user.user_id
      );
    }
  };

  onClickSelectAll = () => {
    this.selectAll = !this.selectAll;

    if (this.selectAll) {
      this.selectedUsers = this.users.map((user) => user.user_id);
    } else {
      this.selectedUsers = [];
    }
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
}
