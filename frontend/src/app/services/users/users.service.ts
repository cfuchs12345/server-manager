import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable, map } from 'rxjs';
import { User, UserInitialPassword, UserPasswordHash, UserToken } from './types';
import { ErrorService } from '../errors/error.service';
import { BehaviorSubject } from 'rxjs';
import { defaultHeadersForJSON } from '../common';
import { AuthenticationService } from '../auth/authentication.service';
import { EncryptionService } from '../encryption/encryption.service';
import { OneTimeKey } from '../auth/types';

@Injectable({ providedIn: 'root' })
export class UserService {
  private _users = new BehaviorSubject<User[]>([]);
  private _initialPassword = new BehaviorSubject<UserInitialPassword | null | undefined>(undefined);

  private dataStore: { users: User[]} = {
    users: [],
  };

  readonly users = this._users.asObservable();
  readonly initialPassword = this._initialPassword.asObservable();

  constructor(private http: HttpClient, private errorService: ErrorService, private encryptionService: EncryptionService) {}

  loadUsers = async () => {
      this.http.get<User[]>('/backend/users').subscribe({
        next: (loadedUsers) => {
          this.dataStore.users = loadedUsers;
        },
        error: (err: any) => {
          this.errorService.newError('User-Service', undefined, err.message);
        },
        complete: () => {
          this.publishUsers();
        },
      });
    }

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

          this._initialPassword.next(new UserInitialPassword(user.user_id, response));
        },
        error: (err: any) => {
          this.errorService.newError("User-Service", user.user_id, err.message);
        },
        complete: () => {
          setTimeout(this.publishUsers, 500);

        },
      });
    }

    deleteUsers = (usersToDelete: User[]) => {
      for (const [i, user] of usersToDelete.entries()) {
        this.http
          .delete<any>('/backend/users/' + user.user_id, {
            headers: defaultHeadersForJSON(),
          })
          .subscribe({
            next: (res) => {
              const indexToDelete = this.dataStore.users.findIndex(u => u.user_id === user.user_id);
              this.dataStore.users.splice(indexToDelete, 1);
            },
            error: (err: any) => {
              this.errorService.newError("User-Service", user.user_id, err.message);
            },
            complete: () => {
              if (usersToDelete[usersToDelete.length -1].user_id === user.user_id) {
                setTimeout(this.publishUsers, 500);
              }
            },
          });
      }
    }

    changePassword = (userId: string, oldPassword: string, newPassword: string, otk: OneTimeKey) => {
      const secret = this.encryptionService.makeSecret(userId, otk.key);
      const body = JSON.stringify( {
        "old_password" : this.encryptionService.encrypt(oldPassword, secret),
        "new_password": this.encryptionService.encrypt(newPassword, secret)
      });
      this.http
      .put<any>('/backend/user/' + userId + "/changepassword", body,  {
        headers: defaultHeadersForJSON(),
      }).subscribe( {
        next: (res) => {

        },
        error: (err: any) => {
          this.errorService.newError("User-Service", userId, err.message);
        },
        complete: () => {

        },
    });
    }

    confirmInitialPasswordReceived = () => {
      this._initialPassword.next(undefined);
    }

    private publishUsers = () => {
      this._users.next(
        this.dataStore.users.slice()
      );
    }
}
