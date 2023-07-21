import { TestBed } from '@angular/core/testing';
import { provideMockStore, MockStore } from '@ngrx/store/testing';
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
import { ToastrModule } from 'ngx-toastr';
import { State } from 'src/app/state';
import { Store } from '@ngrx/store';
import { Server } from './types';

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
  // eslint-disable-next-line  @typescript-eslint/no-unused-vars
  let mockStore: MockStore<State>;

  const testServer = getTestServer();

  const servers: { [ip: string]: Server } = {};
  servers[testServer.ipaddress] = testServer;

  beforeEach(() => {
    TestBed.configureTestingModule({
      imports: [
        LoggerTestingModule,
        HttpClientTestingModule,
        ToastrModule.forRoot(),
      ],
      providers: [
        ErrorService,
        EncryptionService,
        NGXLoggerMock,
        provideMockStore({
          initialState: {
            server: { ids: [testServer.ipaddress], entities: servers },
            plugin: { ids: [], entities: {} },
            conditioncheckresult: { ids: [], entities: {} },
            disabled_plugins: { ids: [], entities: {} },
            notification: { ids: [], entities: {} },
            status: { ids: [], entities: {} },
            user: { ids: [], entities: {} },
            usertoken: { ids: [], entities: {} },
          } as State,
        }),
      ],
    });
    mockStore = TestBed.inject(Store) as MockStore<State>;
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
    obs.subscribe({
      next(value) {
        expect(value).toBeTruthy();
        expect(value).toEqual(testServer);
        expect(value.name).toEqual(testServer.name);
      },
    });

    const req = httpTestingController.expectOne(
      `/backend/servers/${ipaddress}?full_data=false`
    );

    // Assert that the request is a GET.
    expect(req.request.method).toEqual('GET');

    req.flush(testServer);
  });
});
