import { Component, OnInit, OnDestroy } from '@angular/core';
import { FormControl, Validators } from '@angular/forms';
import { MatDialog } from '@angular/material/dialog';
import { Subscription } from 'rxjs';
import { User } from 'src/app/services/users/types';
import { UserService } from 'src/app/services/users/users.service';
import { ConfirmDialogComponent } from 'src/app/ui/confirm-dialog/confirm-dialog.component';
import { MessageDialogComponent } from 'src/app/ui/message_dialog/message-dialog.component';

@Component({
  selector: 'app-configure-users-modal',
  templateUrl: './configure-users-modal.component.html',
  styleUrls: ['./configure-users-modal.component.scss'],
})
export class ConfigureUsersModalComponent
  implements OnInit, OnDestroy
{
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

  users: User[] = [];

  selectedUsers: string[] = [];
  usersSubscription: Subscription | undefined = undefined;
  initialPasswordSubscription: Subscription | undefined = undefined;

  selectAll = false;

  constructor(private userService: UserService, private dialog: MatDialog) {}

  ngOnInit(): void {
    this.usersSubscription = this.userService.users.subscribe((users) => {
      if (users) {
        this.users = users;
      } else {
        // clear messages when empty message received
        this.users = [];
      }
    });
    this.initialPasswordSubscription =
      this.userService.initialPassword.subscribe((passwd) => {
        if (passwd) {
          this.userService.confirmInitialPasswordReceived();
          this.dialog.open(MessageDialogComponent, {
            data: {
              title: 'Initial Password',
              message:
                'The initial password for user ' +
                passwd.user_id +
                ' is: "' +
                passwd.password +
                '"',
            },
          });
        }
      });
    this.userService.loadUsers();
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
      this.userService.saveUser(
        new User(this.userId.value, this.fullName.value, this.email.value)
      ,false);
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

    if (this.selectAll && this.users) {
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
