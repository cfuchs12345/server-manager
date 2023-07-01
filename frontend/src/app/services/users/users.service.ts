import { Injectable } from '@angular/core';
import { HttpClient, HttpHeaders } from '@angular/common/http';
import { User, UserInitialPassword } from './types';
import { ErrorService, Source } from '../errors/error.service';
import { BehaviorSubject } from 'rxjs';
import { defaultHeadersForJSON } from '../common';
import { EncryptionService } from '../encryption/encryption.service';
import { OneTimeKey } from '../auth/types';

@Injectable({ providedIn: 'root' })
export class UserService {
  private _users = new BehaviorSubject<User[]>([]);
  private _initialPassword = new BehaviorSubject<
    UserInitialPassword | null | undefined
  >(undefined);

  private dataStore: { users: User[] } = {
    users: [],
  };

  readonly users = this._users.asObservable();
  readonly initialPassword = this._initialPassword.asObservable();

  constructor(
    private http: HttpClient,
    private errorService: ErrorService,
    private encryptionService: EncryptionService
  ) {}

  loadUsers = async () => {
    this.http.get<User[]>('/backend/users').subscribe({
      next: (loadedUsers) => {
        this.dataStore.users = loadedUsers;
      },
      error: (err) => {
        this.errorService.newError(Source.UserService, undefined, err);
      },
      complete: () => {
        this.publishUsers();
      },
    });
  };

  saveUser = (user: User, firstUser: boolean) => {
    const body = JSON.stringify(user);

    const url = firstUser ? '/backend_nt/users_first' : '/backend/users';

    this.http
      .post<string | undefined | null>(url, body, {
        headers: defaultHeadersForJSON(),
      })
      .subscribe({
        next: (response) => {
          this.dataStore.users.push(user);

          this._initialPassword.next(
            new UserInitialPassword(user.user_id, response)
          );
        },
        error: (err) => {
          this.errorService.newError(Source.UserService, user.user_id, err);
        },
        complete: () => {
          setTimeout(this.publishUsers, 500);
        },
      });
  };

  deleteUsers = (usersToDelete: User[]) => {
    for (const [i, user] of usersToDelete.entries()) {
      const subscription = this.http
        .delete('/backend/users/' + user.user_id, {
          headers: defaultHeadersForJSON(),
        })
        .subscribe({
          next: (res) => {
            const indexToDelete = this.dataStore.users.findIndex(
              (u) => u.user_id === user.user_id
            );
            this.dataStore.users.splice(indexToDelete, 1);
          },
          error: (err) => {
            this.errorService.newError(Source.UserService, user.user_id, err);
          },
          complete: () => {
            if (
              usersToDelete[usersToDelete.length - 1].user_id === user.user_id
            ) {
              setTimeout(this.publishUsers, 500);
            }

            subscription.unsubscribe();
          },
        });
    }
  };

  changePassword = (
    userId: string,
    oldPassword: string,
    newPassword: string,
    otk: OneTimeKey
  ) => {
    const secret = this.encryptionService.makeSecret(userId, otk.key);
    const body = JSON.stringify({
      user_id: userId,
      old_password: this.encryptionService.encrypt(oldPassword, secret),
      new_password: this.encryptionService.encrypt(newPassword, secret),
    });

    const headers = new HttpHeaders({
      'Content-Type': 'application/json',
      'X-custom': `${otk.id}`,
    });

    const subscription = this.http
      .put('/backend/user/' + userId + '/changepassword', body, {
        headers: headers,
      })
      .subscribe({
        error: (err) => {
          this.errorService.newError(Source.UserService, userId, err);
        },
        complete: () => {
          subscription.unsubscribe();
        },
      });
  };

  confirmInitialPasswordReceived = () => {
    this._initialPassword.next(undefined);
  };

  private publishUsers = () => {
    this._users.next(this.dataStore.users.slice());
  };
}
