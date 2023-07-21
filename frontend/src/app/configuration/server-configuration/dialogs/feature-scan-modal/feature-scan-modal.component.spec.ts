import { ComponentFixture, TestBed } from '@angular/core/testing';

import { FeatureScanModalComponent } from './feature-scan-modal.component';
import { MatDialogRef } from '@angular/material/dialog';
import { ServerDiscoveryService } from 'src/app/services/servers/server-discovery.service';
import { ServerService } from 'src/app/services/servers/server.service';
import { HttpClientTestingModule } from '@angular/common/http/testing';
import { ErrorService } from 'src/app/services/errors/error.service';
import { LoggerTestingModule, NGXLoggerMock } from 'ngx-logger/testing';
import { EncryptionService } from 'src/app/services/encryption/encryption.service';
import { MatTableModule } from '@angular/material/table';
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';
import { Store } from '@ngrx/store';
import { MockStore, provideMockStore } from '@ngrx/store/testing';
import { State } from 'src/app/state';
import { ToastrModule } from 'ngx-toastr';

describe('FeatureScanModalComponent', () => {
  let component: FeatureScanModalComponent;
   // eslint-disable-next-line  @typescript-eslint/no-unused-vars
   let logger: NGXLoggerMock;
  let fixture: ComponentFixture<FeatureScanModalComponent>;
  // eslint-disable-next-line  @typescript-eslint/no-unused-vars
  let discoveryService: ServerDiscoveryService;
  // eslint-disable-next-line  @typescript-eslint/no-unused-vars
  let serverService: ServerService;
  // eslint-disable-next-line  @typescript-eslint/no-unused-vars
  let mockStore: MockStore<State>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [
        HttpClientTestingModule,
        LoggerTestingModule,
        MatTableModule,
        BrowserAnimationsModule,
        FeatureScanModalComponent,
        ToastrModule.forRoot()
      ],
      providers: [
        Store,
        ErrorService,
        EncryptionService,
        {
          provide: MatDialogRef,
          useValue: {},
        },
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
        }),
        NGXLoggerMock
      ],
    }).compileComponents();

    fixture = TestBed.createComponent(FeatureScanModalComponent);
    component = fixture.componentInstance;
    // eslint-disable-next-line  @rx-angular/no-explicit-change-detection-apis
    fixture.detectChanges();

    logger = TestBed.inject(NGXLoggerMock);
    discoveryService = TestBed.inject(ServerDiscoveryService);
    serverService = TestBed.inject(ServerService);
    mockStore = TestBed.inject(Store) as MockStore<State>;
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
