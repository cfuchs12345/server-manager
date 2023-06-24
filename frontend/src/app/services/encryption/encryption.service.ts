import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs';
import cryptoRandomString from 'crypto-random-string';
import {PBKDF2, AES, Hex, mode, Word32Array, Utf8, CipherParams} from "jscrypto/es6";
import {
  randomBytes,
  pbkdf2Sync,
  createCipheriv,
  createDecipheriv,
} from 'crypto';

import { Buffer } from 'buffer';
import { OneTimeKey } from '../auth/types';

window.Buffer = window.Buffer || Buffer;

const ALGORITHM = 'aes-256-gcm';
const IV_LENGTH = 16;
const SALT_LENGTH = 64;
const TAG_LENGTH = 16;

const SHA = 'sha256';
const BASE64 = 'base64';
const UTF8 = 'utf8';

@Injectable()
export class EncryptionService {
  constructor(private http: HttpClient) {}


  private getKey(salt: Buffer, secret: string) {
    return pbkdf2Sync(secret, salt, 100000, 32, SHA);
  }

  encrypt = (plainText: string, secret: string): string => {

    {
    const iv = randomBytes(IV_LENGTH);
    const salt = randomBytes(SALT_LENGTH);
    const key = this.getKey(salt, secret);
    const cipher = createCipheriv(ALGORITHM, key, iv);
    const encrypted = Buffer.concat([
      cipher.update(String(plainText), UTF8),
      cipher.final(),
    ]);

    const tag = cipher.getAuthTag();
    const old = Buffer.concat([salt, iv, encrypted, tag]).toString(BASE64);

  }




    const salt = cryptoRandomString({length:SALT_LENGTH});
    const iv =   cryptoRandomString({length:IV_LENGTH});
    const key = PBKDF2.getKey(secret, Hex.parse(salt), {keySize: 256/32, iterations: 100000});
    const authData = Utf8.parse("server-manager");

    var encrypted = AES.encrypt(plainText, key, { iv: Hex.parse(iv), mode: mode.GCM });
    var authTag = mode.GCM.mac(AES, key, Hex.parse(iv), authData, encrypted.cipherText, TAG_LENGTH);
    console.log(salt);
    console.log(iv);
    console.log(key.toString());
    console.log(encrypted.toString());
    console.log(authTag.toString());
    const str = salt.toString() + iv.toString() +  encrypted.toString() + authTag.toString();
    return btoa(str);
  }

  decrypt = (cipherText: string, secret: string): string => {
    const stringValue = atob(cipherText);


    const salt = stringValue.slice(0, SALT_LENGTH);
    const iv = Hex.parse(stringValue.slice(SALT_LENGTH, SALT_LENGTH + IV_LENGTH));
    const encrypted = stringValue.slice(
      SALT_LENGTH + IV_LENGTH,
      -TAG_LENGTH
    );



    const key = PBKDF2.getKey(secret, salt, {keySize: 256/32, iterations: 10000});

    const decrypted =  AES.decrypt(encrypted, key, { iv: iv, mode: mode.GCM }).toString();

    console.log(decrypted);

    return decrypted;
  }

  makeSecret = (uid: string, otk: string): string => {
    const fp = uid.length > 5 ? uid.slice(uid.length - 5, uid.length) : uid;
    const sp = otk.slice(0, otk.length - fp.length);
    return fp + sp;
  };

  requestOneTimeKey = (): Observable<OneTimeKey> => {
    return this.http.get<OneTimeKey>('backend_nt/users/authenticate/otk');
  };
}
