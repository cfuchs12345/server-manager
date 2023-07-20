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

  beforeEach(async () => {
    await TestBed.configureTestingModule({
    providers: [
        ErrorService,
        EncryptionService,
        {
            provide: MatDialogRef,
            useValue: {},
        },
    ],
    imports: [MatFormFieldModule, MatInputModule, LoggerTestingModule, HttpClientTestingModule, MatTableModule, BrowserAnimationsModule, MatFormFieldModule, ReactiveFormsModule, FormsModule, AutodiscoverServerModalComponent],
}).compileComponents();

    fixture = TestBed.createComponent(AutodiscoverServerModalComponent);
    component = fixture.componentInstance;

    // eslint-disable-next-line  @rx-angular/no-explicit-change-detection-apis
    fixture.detectChanges();

    serverService = TestBed.inject(ServerService);
    discoverService = TestBed.inject(ServerDiscoveryService);
    generalService = TestBed.inject(GeneralService);
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
