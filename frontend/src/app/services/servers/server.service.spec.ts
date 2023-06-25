import { TestBed } from '@angular/core/testing';
import { HttpClientTestingModule}  from '@angular/common/http/testing';
import { LoggerTestingModule, NGXLoggerMock } from 'ngx-logger/testing';
import { ServerService } from './server.service';
import { ErrorService } from '../errors/error.service';
import { HttpClient } from '@angular/common/http';
import { EncryptionService } from '../encryption/encryption.service';
import { AuthenticationService } from '../auth/authentication.service';

describe('ServerServiceService', () => {
  let service: ServerService;
  let logger: NGXLoggerMock;
  let http: HttpClient;
  let errorService: ErrorService;
  let encryptionService: EncryptionService;
  let authService: AuthenticationService;

  beforeEach(() => {
    TestBed.configureTestingModule({
      imports: [
        LoggerTestingModule,
        HttpClientTestingModule
      ],
      providers: [
        ErrorService,
        EncryptionService,
        NGXLoggerMock
      ]
    });
    service = TestBed.inject(ServerService);
    logger = TestBed.inject(NGXLoggerMock);
    http = TestBed.inject(HttpClient);
    errorService = TestBed.inject(ErrorService);
    encryptionService = TestBed.inject(EncryptionService);
    authService = TestBed.inject(AuthenticationService);



  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
