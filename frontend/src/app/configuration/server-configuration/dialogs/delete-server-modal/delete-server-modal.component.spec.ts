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

describe('DeleteServerModalComponent', () => {
  let component: DeleteServerModalComponent;
  let fixture: ComponentFixture<DeleteServerModalComponent>;
    // eslint-disable-next-line  @typescript-eslint/no-unused-vars
  let serverService: ServerService;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ DeleteServerModalComponent ],
      imports: [LoggerTestingModule, HttpClientTestingModule, MatFormFieldModule, MatTableModule, BrowserAnimationsModule, MatSelectModule, MatInputModule, ReactiveFormsModule, FormsModule],
      providers: [
        ErrorService, EncryptionService,
        {
          provide: MatDialog,
          useValue: {}
        },
        {
          provide: MatDialogRef,
          useValue: {}
        }

      ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(DeleteServerModalComponent);
    component = fixture.componentInstance;

    // eslint-disable-next-line  @rx-angular/no-explicit-change-detection-apis
    fixture.detectChanges();

    serverService= TestBed.inject(ServerService);
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
