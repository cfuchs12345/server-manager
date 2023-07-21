import { TestBed } from '@angular/core/testing';

import { PluginService } from './plugin.service';
import { ErrorService } from '../errors/error.service';
import { LoggerTestingModule } from 'ngx-logger/testing';
import { HttpClientTestingModule } from '@angular/common/http/testing';
import { MockStore, provideMockStore } from '@ngrx/store/testing';
import { State } from 'src/app/state';
import { Store } from '@ngrx/store';
import { EncryptionService } from '../encryption/encryption.service';
import { ToastrModule } from 'ngx-toastr';


describe('PluginService', () => {
  let service: PluginService;
  // eslint-disable-next-line  @typescript-eslint/no-unused-vars
  let errorService: ErrorService;
 // eslint-disable-next-line  @typescript-eslint/no-unused-vars
 let mockStore: MockStore<State>;

  beforeEach(() => {
    TestBed.configureTestingModule({
      providers: [ErrorService, EncryptionService,
        provideMockStore({
          initialState: {
            server: { ids: [], entities: {} },
            plugin: { ids: [], entities: {} },
            conditioncheckresult: { ids: [], entities: {} },
            disabled_plugins: { ids: [], entities: {} },
            notification: { ids: [], entities: {} },
            status: { ids: [], entities: {} },
            user: { ids: [], entities: {} },
            usertoken: { ids: [], entities: {} },
          } as State,
        }),],
      imports: [LoggerTestingModule, HttpClientTestingModule, ToastrModule.forRoot(),]
    });
    service = TestBed.inject(PluginService);
    errorService = TestBed.inject(ErrorService);
    mockStore = TestBed.inject(Store) as MockStore<State>;
  });

  it('should be created', () => {
    expect(service).toBeTruthy();
  });
});
