import { Injectable } from '@angular/core';
import { Router } from '@angular/router';
import { HttpClient, HttpHeaders } from '@angular/common/http';
import { BehaviorSubject, Observable } from 'rxjs';
import { map, mergeMap } from 'rxjs/operators';
import { User, UserToken } from '../users/types';
import { OneTimeKey } from './types';
import { ErrorService } from '../errors/error.service';
import { EncryptionService } from '../encryption/encryption.service';

@Injectable({ providedIn: 'root' })
export class AuthenticationService {
  private _userTokenSubject: BehaviorSubject<UserToken | null> =
    new BehaviorSubject<UserToken | null>(null);
  public readonly userTokenSubject = this._userTokenSubject.asObservable();
  public userToken: UserToken | undefined = undefined;

  constructor(
    private router: Router,
    private http: HttpClient,
    private errorService: ErrorService,
    private encryptionService: EncryptionService
  ) {}

  login(userId: string, password: string): Observable<UserToken> {
    return this.requestOneTimeKey().pipe(
      mergeMap((otk) => {
        var auth = 'Basic ' + btoa( userId + ':' + this.encryptionService.encrypt(password, this.encryptionService.makeSecret(userId, otk.key)));

        const headers = new HttpHeaders({
          'Content-Type': 'application/json',
          Authorization: `${auth}`,
          'X-custom': `${otk.id}`,
        });

        return this.http
          .post<UserToken>(`backend_nt/users/authenticate`, '', {
            headers: headers,
          })
          .pipe(
            map((userToken) => {
              this.userToken = userToken;
              this._userTokenSubject.next(userToken);
              return userToken;
            })
          );
      })
    );
  }

  requestOneTimeKey(): Observable<OneTimeKey> {
    return this.http.get<OneTimeKey>('backend_nt/users/authenticate/otk');
  }

  userExist = (): Observable<boolean> => {
    return this.http.get<boolean>('/backend_nt/users/exist');
  }

  logout() {
    this._userTokenSubject.next(null);
    this.router.navigate(['/login']);
  }
}
