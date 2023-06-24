import { Injectable } from '@angular/core';
import { HttpClient, HttpHeaders } from '@angular/common/http';
import { User, UserInitialPassword } from './types';
import { ErrorService, Source } from '../errors/error.service';
import { BehaviorSubject, Observable, catchError, map, tap, throwError } from 'rxjs';
import { defaultHeadersForJSON } from '../common';
import { EncryptionService } from '../encryption/encryption.service';
import { OneTimeKey } from '../auth/types';

@Injectable({ providedIn: 'root' })
export class UserService {
  private _users = new BehaviorSubject<User[]>([]);

  private dataStore: { users: User[] } = {
    users: [],
  };

  readonly users = this._users.asObservable();

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
      error: (err: any) => {
        this.errorService.newError(Source.UserService, undefined, err);
      },
      complete: () => {
        setTimeout(this.publishUsers, 50);
      },
    });
  };

  saveUser = (user: User, firstUser: boolean) : Observable<UserInitialPassword> => {
    const body = JSON.stringify(user);

    const url = firstUser ? '/backend_nt/users_first' : '/backend/users';

    return this.http
      .post<string | null>(url, body, {
        headers: defaultHeadersForJSON(),
      })
      .pipe(
        tap( (response) =>  this.dataStore.users.push(user) ),
        map( (response) => new UserInitialPassword(user.user_id, response)),
        catchError( (err) => {this.errorService.newError(Source.UserService, user.user_id, err);  return throwError( () => err);}),
        tap( (reponse) => setTimeout(this.publishUsers, 50) )
      );
  };

  deleteUsers = (usersToDelete: User[]) => {
    for (const [i, user] of usersToDelete.entries()) {
      this.http
        .delete<any>('/backend/users/' + user.user_id, {
          headers: defaultHeadersForJSON(),
        })
        .subscribe({
          next: (res) => {
            const indexToDelete = this.dataStore.users.findIndex(
              (u) => u.user_id === user.user_id
            );
            this.dataStore.users.splice(indexToDelete, 1);
          },
          error: (err: any) => {
            this.errorService.newError(Source.UserService, user.user_id, err);
          },
          complete: () => {
            if (
              usersToDelete[usersToDelete.length - 1].user_id === user.user_id
            ) {
              setTimeout(this.publishUsers, 50);
            }
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

    this.http
      .put<any>('/backend/user/' + userId + '/changepassword', body, {
        headers: headers,
      })
      .subscribe({
        next: (res) => {},
        error: (err: any) => {
          this.errorService.newError(Source.UserService, userId, err);
        },
        complete: () => {},
      });
  };

  private publishUsers = () => {
    this._users.next(this.dataStore.users.slice());
  };
}
