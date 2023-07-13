import { Injectable } from '@angular/core';
import { HttpClient, HttpHeaders } from '@angular/common/http';
import { User, UserInitialPassword } from './types';
import { ErrorService, Source } from '../errors/error.service';
import { Observable, catchError, map, throwError, filter } from 'rxjs';
import { defaultHeadersForJSON } from '../common';
import { EncryptionService } from '../encryption/encryption.service';
import { OneTimeKey } from '../auth/types';
import { Store } from '@ngrx/store';
import { removeOne } from 'src/app/state/user/user.actions';
import { NGXLogger } from 'ngx-logger';
import { EventService } from '../events/event.service';
import { Event } from '../events/types';

@Injectable({ providedIn: 'root' })
export class UserService {
  constructor(
    private store: Store,
    private http: HttpClient,
    private errorService: ErrorService,
    private eventService: EventService,
    private encryptionService: EncryptionService,
    private logger: NGXLogger
  ) {
    this.eventService.eventSubject$
      .pipe(
        filter((eventAndObject: [Event, User]) => {
          const event = eventAndObject[0];

          return event.object_type === 'User';
        })
      )
      .subscribe((eventAndObject: [Event, User]) => {
        const event = eventAndObject[0];

        if (event.event_type === 'Insert' || event.event_type === 'Update') {
          this.listUsers();
        } else if (event.event_type === 'Delete') {
          this.store.dispatch(removeOne({ user_id: event.key }));
        }
      });
  }

  listUsers = (): Observable<User[]> => {
    return this.http.get<User[]>('/backend/users');
  };

  saveUser = (
    user: User,
    firstUser: boolean
  ): Observable<UserInitialPassword> => {
    const body = JSON.stringify(user);

    const url = firstUser ? '/backend_nt/users_first' : '/backend/users';

    return this.http
      .post<string | undefined | null>(url, body, {
        headers: defaultHeadersForJSON(),
      })
      .pipe(
        catchError((err) => {
          this.errorService.newError(Source.UserService, user.user_id, err);
          return throwError(() => err);
        }),
        map((response) => {
          return new UserInitialPassword(user.user_id, response);
        })
      );
  };

  deleteUsers = (usersToDelete: User[]) => {
    for (const [, user] of usersToDelete.entries()) {
      const subscription = this.http
        .delete('/backend/users/' + user.user_id, {
          headers: defaultHeadersForJSON(),
        })
        .subscribe({
          next: () => {
            this.store.dispatch(removeOne({ user_id: user.user_id }));
          },
          error: (err) => {
            this.errorService.newError(Source.UserService, user.user_id, err);
          },
          complete: () => {
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
}
