import { ComponentFixture, TestBed } from '@angular/core/testing';
import { HttpClientTestingModule } from '@angular/common/http/testing';
import { ListPluginsModalComponent } from './list-plugins-modal.component';
import { PluginService } from 'src/app/services/plugins/plugin.service';
import { ErrorService } from 'src/app/services/errors/error.service';
import { MatTableModule } from '@angular/material/table';
import { State } from 'src/app/state';
import { MockStore, provideMockStore } from '@ngrx/store/testing';
import { Store } from '@ngrx/store';
import { LoggerTestingModule, NGXLoggerMock } from 'ngx-logger/testing';
import { EncryptionService } from 'src/app/services/encryption/encryption.service';
import { ToastrModule } from 'ngx-toastr';

describe('ListPluginsModalComponent', () => {
  let component: ListPluginsModalComponent;
  let fixture: ComponentFixture<ListPluginsModalComponent>;
  // eslint-disable-next-line  @typescript-eslint/no-unused-vars
  let servicePlugins: PluginService;
  // eslint-disable-next-line  @typescript-eslint/no-unused-vars
  let errorService: ErrorService;
  // eslint-disable-next-line  @typescript-eslint/no-unused-vars
  let mockStore: MockStore<State>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [
        HttpClientTestingModule,
        MatTableModule,
        ListPluginsModalComponent,
        LoggerTestingModule,
        ToastrModule.forRoot()
      ],
      providers: [
        ErrorService,
        EncryptionService,
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

    fixture = TestBed.createComponent(ListPluginsModalComponent);
    component = fixture.componentInstance;

    // eslint-disable-next-line  @rx-angular/no-explicit-change-detection-apis
    fixture.detectChanges();

    servicePlugins = TestBed.inject(PluginService);
    errorService = TestBed.inject(ErrorService);
    mockStore = TestBed.inject(Store) as MockStore<State>;
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
