import { Injectable } from '@angular/core';
import { HttpClient, HttpHeaders } from '@angular/common/http';
import { User, UserInitialPassword } from './types';
import { ErrorService, Source } from '../errors/error.service';
import { Observable, catchError, map, throwError, take } from 'rxjs';
import { defaultHeadersForJSON } from '../common';
import { EncryptionService } from '../encryption/encryption.service';
import { OneTimeKey } from '../auth/types';
import { Store } from '@ngrx/store';
import { addOne, removeOne, upsertOne } from 'src/app/state/user/user.actions';
import { NGXLogger } from 'ngx-logger';
import {
  EventHandler,
  EventHandlingFunction,
  EventHandlingGetObjectFunction,
  EventHandlingUpdateFunction,
} from '../events/types';
import { EventType } from '../events/types';
import { selectUserByUserId } from 'src/app/state/user/user.selectors';
import { EventService } from '../events/event.service';

@Injectable({ providedIn: 'root' })
export class UserService {
  // eslint-disable-next-line  @typescript-eslint/no-unused-vars
  insertEventFunction: EventHandlingFunction<User> = (
    eventType: EventType,
    keyType: string,
    key: string,
    data: string,
    object: User
  ) => {
    this.update(key, eventType, object);
  };

  updateEventFunction: EventHandlingUpdateFunction<User> = (
    eventType: EventType,
    keyType: string,
    key: string,
    data: string,
    version: number,
    object: User
  ) => {
    const server$ = this.store.select(selectUserByUserId(key));
    server$.pipe(take(1)).subscribe({
      next: (user) => {
        // only update, if version is different or if the current object in store is preliminary
        if (user) {
          this.update(key, eventType, object);
        }
      },
    });
  };

  // eslint-disable-next-line  @typescript-eslint/no-unused-vars
  deleteEventFunction: EventHandlingFunction<User> = (
    eventType: EventType,
    key_name: string,
    key: string,
    data: string // eslint-disable-line @typescript-eslint/no-unused-vars
  ) => {
    this.store.dispatch(removeOne({ user_id: key }));
  };

  getObjectFunction: EventHandlingGetObjectFunction<User> = (
    key_name: string,
    key: string
  ): Observable<User> => {
    return this.getUser(key);
  };

  constructor(
    private store: Store,
    private http: HttpClient,
    private errorService: ErrorService,
    private eventService: EventService,
    private encryptionService: EncryptionService,
    private logger: NGXLogger
  ) {
    this.eventService.registerEventHandler(
      new EventHandler<User>(
        'User',
        this.insertEventFunction,
        this.updateEventFunction,
        this.deleteEventFunction,
        this.getObjectFunction
      )
    );
  }

  update = (
    userId: string,
    event_type: 'Insert' | 'Update' | 'Refresh' | 'Delete',
    object: User
  ) => {
    if (event_type === 'Insert') {
      this.store.dispatch(addOne({ user: object }));
    } else {
      this.store.dispatch(upsertOne({ user: object }));
    }
  };

  getUser = (userId: string): Observable<User> => {
    return this.http.get<User>(`/backend/users/${userId}`);
  };

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
