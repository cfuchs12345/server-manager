import { ComponentFixture, TestBed } from '@angular/core/testing';

import { AutodiscoverServerModalComponent } from './autodiscover-server-modal.component';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule } from '@angular/material/input';
import { ErrorService } from 'src/app/services/errors/error.service';
import { MatDialogRef } from '@angular/material/dialog';
import { ServerService } from 'src/app/services/servers/server.service';
import { ServerDiscoveryService } from 'src/app/services/servers/server-discovery.service';
import { GeneralService } from 'src/app/services/general/general.service';
import { LoggerTestingModule } from 'ngx-logger/testing';
import { HttpClientTestingModule } from '@angular/common/http/testing';
import { EncryptionService } from 'src/app/services/encryption/encryption.service';
import { FormsModule, ReactiveFormsModule } from '@angular/forms';
import { MatTableModule } from '@angular/material/table';
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';
import { State } from 'src/app/state';
import { MockStore, provideMockStore } from '@ngrx/store/testing';
import { Store } from '@ngrx/store';
import { ToastrModule } from 'ngx-toastr';

describe('AutodiscoverServerModalComponent', () => {
  let component: AutodiscoverServerModalComponent;
  let fixture: ComponentFixture<AutodiscoverServerModalComponent>;

  // eslint-disable-next-line  @typescript-eslint/no-unused-vars
  let serverService: ServerService;
  // eslint-disable-next-line  @typescript-eslint/no-unused-vars
  let discoverService: ServerDiscoveryService;
  // eslint-disable-next-line  @typescript-eslint/no-unused-vars
  let generalService: GeneralService;
  // eslint-disable-next-line  @typescript-eslint/no-unused-vars
  let errorService: ErrorService;
  // eslint-disable-next-line  @typescript-eslint/no-unused-vars
  let mockStore: MockStore<State>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      providers: [
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
      ],
      imports: [
        MatFormFieldModule,
        MatInputModule,
        LoggerTestingModule,
        HttpClientTestingModule,
        MatTableModule,
        BrowserAnimationsModule,
        MatFormFieldModule,
        ReactiveFormsModule,
        FormsModule,
        AutodiscoverServerModalComponent,
        ToastrModule.forRoot()
      ],
    }).compileComponents();

    fixture = TestBed.createComponent(AutodiscoverServerModalComponent);
    component = fixture.componentInstance;

    // eslint-disable-next-line  @rx-angular/no-explicit-change-detection-apis
    fixture.detectChanges();

    serverService = TestBed.inject(ServerService);
    errorService = TestBed.inject(ErrorService);
    discoverService = TestBed.inject(ServerDiscoveryService);
    generalService = TestBed.inject(GeneralService);
    mockStore = TestBed.inject(Store) as MockStore<State>;
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
