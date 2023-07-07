import { TestBed } from '@angular/core/testing';
import {
  HttpClientTestingModule,
  HttpTestingController,
} from '@angular/common/http/testing';
import { LoggerTestingModule, NGXLoggerMock } from 'ngx-logger/testing';
import { ServerService } from './server.service';
import { ErrorService } from '../errors/error.service';
import { HttpClient } from '@angular/common/http';
import { EncryptionService } from '../encryption/encryption.service';
import { AuthenticationService } from '../auth/authentication.service';
import { getTestServer } from 'src/app/test/data';

describe('ServerServiceService', () => {
  let service: ServerService;
  // eslint-disable-next-line  @typescript-eslint/no-unused-vars
  let logger: NGXLoggerMock;
  // eslint-disable-next-line  @typescript-eslint/no-unused-vars
  let http: HttpClient;
  // eslint-disable-next-line  @typescript-eslint/no-unused-vars
  let errorService: ErrorService;
  // eslint-disable-next-line  @typescript-eslint/no-unused-vars
  let encryptionService: EncryptionService;
  // eslint-disable-next-line  @typescript-eslint/no-unused-vars
  let authService: AuthenticationService;

  let httpTestingController: HttpTestingController;

  beforeEach(() => {
    TestBed.configureTestingModule({
      imports: [LoggerTestingModule, HttpClientTestingModule],
      providers: [ErrorService, EncryptionService, NGXLoggerMock],
    });
    service = TestBed.inject(ServerService);
    logger = TestBed.inject(NGXLoggerMock);
    http = TestBed.inject(HttpClient);
    errorService = TestBed.inject(ErrorService);
    encryptionService = TestBed.inject(EncryptionService);
    authService = TestBed.inject(AuthenticationService);

    httpTestingController = TestBed.inject(HttpTestingController);
  });

  afterEach(() => {
    // After every test, assert that there are no more pending requests.
    httpTestingController.verify();
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });

  it('should return an Observerable that returns a server', () => {
    const testServer = getTestServer();

    const ipaddress = testServer.ipaddress;

    const obs = service.getServer(ipaddress, false);
    obs.subscribe( {
      next(value) {
          expect(value).toBeTruthy();
          expect(value).toEqual(testServer);
          expect(value.name).toEqual(testServer.name);
      },
    });

    const req = httpTestingController.expectOne(`/backend/servers/${ipaddress}`);

    // Assert that the request is a GET.
    expect(req.request.method).toEqual('GET');

    req.flush(testServer);
  });
});
