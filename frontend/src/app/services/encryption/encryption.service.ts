import { Injectable } from "@angular/core";
import { HttpClient } from '@angular/common/http';
import { Observable } from 'rxjs';
import {randomBytes, pbkdf2Sync, createCipheriv,createDecipheriv } from 'crypto'
import { Buffer } from 'buffer';
import { OneTimeKey } from "../auth/types";

window.Buffer = window.Buffer || Buffer;

const ALGORITHM = 'aes-256-gcm';
const IV_LENGTH = 16;
const SALT_LENGTH = 64;
const TAG_LENGTH = 16;

const SHA = 'sha256';
const BASE64 = 'base64';
const UTF8 = 'utf8'

@Injectable()
export class EncryptionService {

  constructor( private http: HttpClient) {}

  private getKey(salt: Buffer, secret: string) {
    return pbkdf2Sync(secret, salt, 100000, 32, SHA);
  }

  encrypt(plainText: string, secret: string) {
    const iv = randomBytes(IV_LENGTH);
    const salt = randomBytes(SALT_LENGTH);
    const key = this.getKey(salt, secret);
    const cipher = createCipheriv(ALGORITHM, key, iv);
    const encrypted = Buffer.concat([
      cipher.update(String(plainText), UTF8),
      cipher.final(),
    ]);

    const tag = cipher.getAuthTag();
    return Buffer.concat([salt, iv, encrypted, tag]).toString(BASE64);
  }

  decrypt(cipherText: string, secret: string) {
    const stringValue = Buffer.from(String(cipherText), BASE64);

    const salt = stringValue.subarray(0, SALT_LENGTH);
    const iv = stringValue.subarray(SALT_LENGTH, SALT_LENGTH + IV_LENGTH);
    const encrypted = stringValue.subarray(SALT_LENGTH + IV_LENGTH, stringValue.length - TAG_LENGTH);
    const tag = stringValue.subarray(stringValue.length - TAG_LENGTH);
    const key = this.getKey(salt, secret);


    const decipher = createDecipheriv(ALGORITHM, key, iv);

    decipher.setAuthTag(tag);

    return decipher.update(encrypted) + decipher.final(UTF8);
  }


  makeSecret = (uid: string, otk: string): string => {
    const fp = uid.length > 5 ? uid.slice(uid.length-5, uid.length) : uid;
    const sp = otk.slice(0, otk.length - fp.length);
    return fp+sp;
  }


  requestOneTimeKey = (): Observable<OneTimeKey> => {
    return this.http.get<OneTimeKey>('backend_nt/users/authenticate/otk');
  }
}
