import { ComponentFixture, TestBed } from '@angular/core/testing';

import { DeleteServerModalComponent } from './delete-server-modal.component';
import { ServerService } from 'src/app/services/servers/server.service';
import { MatDialog, MatDialogRef } from '@angular/material/dialog';
import { LoggerTestingModule } from 'ngx-logger/testing';
import { HttpClientTestingModule } from '@angular/common/http/testing';
import { ErrorService } from 'src/app/services/errors/error.service';
import { EncryptionService } from 'src/app/services/encryption/encryption.service';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatSelectModule } from '@angular/material/select';
import { MatInputModule } from '@angular/material/input';
import { FormsModule, ReactiveFormsModule } from '@angular/forms';
import { MatTableModule } from '@angular/material/table';
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';
import { MockStore, provideMockStore } from '@ngrx/store/testing';
import { State } from 'src/app/state';
import { Store } from '@ngrx/store';
import { ToastrModule } from 'ngx-toastr';

describe('DeleteServerModalComponent', () => {
  let component: DeleteServerModalComponent;
  let fixture: ComponentFixture<DeleteServerModalComponent>;
    // eslint-disable-next-line  @typescript-eslint/no-unused-vars
  let serverService: ServerService;

   // eslint-disable-next-line  @typescript-eslint/no-unused-vars
   let mockStore: MockStore<State>;


  beforeEach(async () => {
    await TestBed.configureTestingModule({
    imports: [LoggerTestingModule, HttpClientTestingModule, MatFormFieldModule, MatTableModule, BrowserAnimationsModule, MatSelectModule, MatInputModule, ReactiveFormsModule, FormsModule, DeleteServerModalComponent, ToastrModule.forRoot()],
    providers: [
        ErrorService, EncryptionService,
        {
            provide: MatDialog,
            useValue: {}
        },
        {
            provide: MatDialogRef,
            useValue: {}
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
    ]
})
    .compileComponents();

    fixture = TestBed.createComponent(DeleteServerModalComponent);
    component = fixture.componentInstance;

    // eslint-disable-next-line  @rx-angular/no-explicit-change-detection-apis
    fixture.detectChanges();

    serverService= TestBed.inject(ServerService);
    mockStore = TestBed.inject(Store) as MockStore<State>;
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
