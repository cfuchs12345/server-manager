import { ComponentFixture, TestBed } from '@angular/core/testing';

import { FeatureScanModalComponent } from './feature-scan-modal.component';
import { MatDialogRef } from '@angular/material/dialog';
import { ServerDiscoveryService } from 'src/app/services/servers/server-discovery.service';
import { ServerService } from 'src/app/services/servers/server.service';
import { HttpClientTestingModule } from '@angular/common/http/testing';
import { ErrorService } from 'src/app/services/errors/error.service';
import { LoggerTestingModule } from 'ngx-logger/testing';
import { EncryptionService } from 'src/app/services/encryption/encryption.service';
import {  MatTableModule } from '@angular/material/table';
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';

describe('FeatureScanModalComponent', () => {
  let component: FeatureScanModalComponent;
  let fixture: ComponentFixture<FeatureScanModalComponent>;
  // eslint-disable-next-line  @typescript-eslint/no-unused-vars
  let discoveryService: ServerDiscoveryService;
  // eslint-disable-next-line  @typescript-eslint/no-unused-vars
  let serverService: ServerService;


  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ FeatureScanModalComponent ],
      imports: [HttpClientTestingModule, LoggerTestingModule, MatTableModule, BrowserAnimationsModule],
      providers: [
        ErrorService,
        EncryptionService,
        {
          provide: MatDialogRef,
          useValue: {}
        }
      ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(FeatureScanModalComponent);
    component = fixture.componentInstance;
    // eslint-disable-next-line  @rx-angular/no-explicit-change-detection-apis
    fixture.detectChanges();

    discoveryService = TestBed.inject(ServerDiscoveryService);
    serverService = TestBed.inject(ServerService);
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
