import { Injectable } from '@angular/core';
import { HttpClient, HttpHeaders } from '@angular/common/http';
import { Observable, throwError } from 'rxjs';
import { catchError, map, mergeMap, tap } from 'rxjs/operators';
import { UserToken } from '../users/types';
import { ErrorService, Source } from '../errors/error.service';
import { EncryptionService } from '../encryption/encryption.service';
import { Router } from '@angular/router';

@Injectable({ providedIn: 'root' })
export class AuthenticationService {
  private userToken: UserToken | undefined = undefined;

  constructor(
    private router: Router,
    private http: HttpClient,
    private errorService: ErrorService,
    private encryptionService: EncryptionService
  ) {}

  login(userId: string, password: string): Observable<UserToken> {
    return this.encryptionService.requestOneTimeKey().pipe(
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
            catchError((err) => {
              this.errorService.newError(
                Source.AuthenticationService,
                undefined,
                err
              );
              return throwError(() => err);
            }),
            map((userToken) => {
              return userToken;
            }),
            tap( (userToken) => {
              this.userToken = userToken;
            })
          );
      })
    );
  }

  logout = () => {
    this.userToken = undefined;
    this.router.navigate(['/login']);
  };

  getUserToken = (): UserToken => {
    return Object.assign([], this.userToken);
  }

  userExist = (): Observable<boolean> => {
    return this.http.get<boolean>('/backend_nt/users/exist');
  };
}
