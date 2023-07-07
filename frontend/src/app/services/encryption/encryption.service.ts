import { Injectable } from '@angular/core';
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs';

import cryptoRandomString from 'crypto-random-string';

import {
  AES,
  SHA256,
  PBKDF2,
  Hex,
  Utf8,
  Latin1,
  mode,
  pad,
  formatter,
} from 'jscrypto/es6';


import { OneTimeKey } from '../auth/types';

// eslint-disable-next-line  @typescript-eslint/no-var-requires
const Buffer = require('buffer/').Buffer

const IV_LENGTH = 16;
const SALT_LENGTH = 64;
const TAG_LENGTH = 16;
const BASE64 = 'base64';
const ROUNDS = 10000;



@Injectable()
export class EncryptionService {
  constructor(private http: HttpClient) {}

  encrypt = (plainText: string, secret: string): string => {
    const iv_hex = Hex.parse(cryptoRandomString({ length: IV_LENGTH * 2 }));
    const salt_hex = Hex.parse(cryptoRandomString({ length: SALT_LENGTH * 2 }));

    const key = PBKDF2.getKey(Utf8.parse(secret), salt_hex, {
      keySize: 256 / 32,
      iterations: ROUNDS,
      Hasher: SHA256,
    });

    const encrypted = AES.encrypt(plainText, key, {
      iv: iv_hex,
      mode: mode.GCM,
      padding: pad.NoPadding,
    });
    const autTag = mode.GCM.mac(
      AES,
      key,
      iv_hex,
      undefined,
      encrypted.cipherText,
      TAG_LENGTH
    );

    if (!encrypted.cipherText || !autTag) {
      return '';
    }

    const txt = Buffer.concat([
      salt_hex.toUint8Array(),
      iv_hex.toUint8Array(),
      encrypted.cipherText.toUint8Array(),
      autTag.toUint8Array(),
    ]).toString(BASE64);

    return txt;
  };

  decrypt = (cipherText: string, secret: string): string => {
    const stringValue = Buffer.from(cipherText, BASE64).toString('hex');

    const salt = stringValue.slice(0, SALT_LENGTH * 2);
    const iv = stringValue.slice(
      SALT_LENGTH * 2,
      (SALT_LENGTH + IV_LENGTH) * 2
    );

    const encrypted = stringValue.slice(
      (SALT_LENGTH + IV_LENGTH) * 2,
      -TAG_LENGTH * 2
    );
    // eslint-disable-next-line  @typescript-eslint/no-unused-vars
    const tag = stringValue.slice(-TAG_LENGTH * 2);

    const key = PBKDF2.getKey(secret, Hex.parse(salt), {
      keySize: 256 / 32,
      iterations: ROUNDS,
      Hasher: SHA256,
    });

    const decrypted = AES.decrypt(
      {
        cipherText: Hex.parse(encrypted),
        formatter: formatter.OpenSSLFormatter,
      },
      key,
      {
        iv: Hex.parse(iv),
        mode: mode.GCM,
        padding: pad.NoPadding,
      }
    );

    return decrypted.toString(Latin1);
  };

  makeSecret = (uid: string, otk: string): string => {
    const fp = uid.length > 5 ? uid.slice(uid.length - 5, uid.length) : uid;
    const sp = otk.slice(0, otk.length - fp.length);
    return fp + sp;
  };

  requestOneTimeKey = (): Observable<OneTimeKey> => {
    return this.http.get<OneTimeKey>('backend_nt/users/authenticate/otk');
  };
}
