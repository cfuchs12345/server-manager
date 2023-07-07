import { ComponentFixture, TestBed } from '@angular/core/testing';

import { ConfigureDnsModalComponent } from './configure-dns-modal.component';
import { GeneralService } from 'src/app/services/general/general.service';
import { HttpClientTestingModule } from '@angular/common/http/testing';
import { ErrorService } from 'src/app/services/errors/error.service';
import { EncryptionService } from 'src/app/services/encryption/encryption.service';
import { LoggerTestingModule } from 'ngx-logger/testing';
import { MatDialog } from '@angular/material/dialog';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatInputModule } from '@angular/material/input';
import { FormsModule, ReactiveFormsModule } from '@angular/forms';
import { MatTableModule } from '@angular/material/table';
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';

describe('ConfigureDnsModalComponent', () => {
  let component: ConfigureDnsModalComponent;
  let fixture: ComponentFixture<ConfigureDnsModalComponent>;
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  let configService: GeneralService;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ConfigureDnsModalComponent],
      imports: [
        HttpClientTestingModule,
        LoggerTestingModule,
        MatTableModule,
        BrowserAnimationsModule,
        MatFormFieldModule,
        MatInputModule,
        FormsModule,
        ReactiveFormsModule,
      ],
      providers: [
        ErrorService,
        EncryptionService,
        {
          provide: MatDialog,
          useValue: {},
        },
      ],
    }).compileComponents();

    fixture = TestBed.createComponent(ConfigureDnsModalComponent);
    component = fixture.componentInstance;
    // eslint-disable-next-line @rx-angular/no-explicit-change-detection-apis
    fixture.detectChanges();

    configService = TestBed.inject(GeneralService);
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
