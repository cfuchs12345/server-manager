import { ComponentFixture, TestBed } from '@angular/core/testing';
import { provideMockStore, MockStore } from '@ngrx/store/testing';
import { DisablePluginsModalComponent } from './disable-plugins-modal.component';
import { HttpClientTestingModule } from '@angular/common/http/testing';
import { PluginService } from 'src/app/services/plugins/plugin.service';
import { ErrorService } from 'src/app/services/errors/error.service';
import { MatTableModule } from '@angular/material/table';
import { LoggerTestingModule } from 'ngx-logger/testing';
import { EncryptionService } from 'src/app/services/encryption/encryption.service';
import { ToastrModule } from 'ngx-toastr';
import { State } from 'src/app/state';
import { Store } from '@ngrx/store';

describe('DisablePluginsModalComponent', () => {
  let component: DisablePluginsModalComponent;
  let fixture: ComponentFixture<DisablePluginsModalComponent>;

  // eslint-disable-next-line  @typescript-eslint/no-unused-vars
  let pluginService: PluginService;
  // eslint-disable-next-line  @typescript-eslint/no-unused-vars
  let errorService: ErrorService;

  // eslint-disable-next-line  @typescript-eslint/no-unused-vars
  let mockStore: MockStore<State>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [
        HttpClientTestingModule,
        MatTableModule,
        DisablePluginsModalComponent,
        LoggerTestingModule,
        ToastrModule.forRoot(),
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
      ],
    }).compileComponents();

    fixture = TestBed.createComponent(DisablePluginsModalComponent);
    component = fixture.componentInstance;

    // eslint-disable-next-line  @rx-angular/no-explicit-change-detection-apis
    fixture.detectChanges();
    mockStore = TestBed.inject(Store) as MockStore<State>;
    pluginService = TestBed.inject(PluginService);
    errorService = TestBed.inject(ErrorService);
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
