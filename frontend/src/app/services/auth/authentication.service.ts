import { Injectable } from '@angular/core';
import { HttpClient, HttpHeaders } from '@angular/common/http';
import { Observable, throwError, take } from 'rxjs';
import { catchError, mergeMap, tap } from 'rxjs/operators';
import { UserToken } from '../users/types';
import { ErrorService, Source } from '../errors/error.service';
import { EncryptionService } from '../encryption/encryption.service';
import { Router } from '@angular/router';
import { Store } from '@ngrx/store';
import { selectToken } from 'src/app/state/usertoken/usertoken.selectors';
import * as GlobalActions from '../../../app/state/global.actions';
import { upsertOneExist } from 'src/app/state/user/user.actions';
import { selectUserExistByKey } from 'src/app/state/user/user.selectors';

@Injectable({ providedIn: 'root' })
export class AuthenticationService {
  userToken$: Observable<UserToken | undefined>;
  userExist$: Observable<boolean | undefined>;

  constructor(
    private store: Store,
    private router: Router,
    private http: HttpClient,
    private errorService: ErrorService,
    private encryptionService: EncryptionService
  ) {
    this.userToken$ = store.select(selectToken());
    this.userExist$ = store.select(selectUserExistByKey());
  }

  login(userId: string, password: string): Observable<UserToken> {
    return this.encryptionService.requestOneTimeKey().pipe(
      take(1),
      mergeMap((otk) => {
        const secret = this.encryptionService.makeSecret(userId, otk.key);
        const encrypted_password = this.encryptionService.encrypt(
          password,
          secret
        );
        const base64_enc = btoa(`${userId}:${encrypted_password}`);

        const headers = new HttpHeaders({
          'Content-Type': 'application/json',
          Authorization: `Basic ${base64_enc}`,
          'X-custom': `${otk.id}`,
        });

        return this.http
          .post<UserToken>(`backend_nt/users/authenticate`, '', {
            headers: headers,
          })
          .pipe(
            take(1),
            catchError((err) => {
              this.errorService.newError(
                Source.AuthenticationService,
                undefined,
                err
              );
              return throwError(() => err);
            })
          );
      })
    );
  }

  logout = () => {
    this.userToken$.pipe(take(1)).subscribe((token) => {
      if (token) {
        this.store.dispatch(GlobalActions.logout({ userToken: token, logout: true }));
      }
    });

    this.router.navigate(['/login']);
  };

  userExist = (): Observable<boolean> => {
    return this.http.get<boolean>('/backend_nt/users/exist')
    .pipe(
      tap((result) => this.store.dispatch(upsertOneExist({exist: result})))
    );
  };
}
