import { ComponentFixture, TestBed } from '@angular/core/testing';

import { ConfigureFeaturesModalComponent } from './configure-features-modal.component';
import { ServerService } from 'src/app/services/servers/server.service';
import { PluginService } from 'src/app/services/plugins/plugin.service';
import { FormBuilder, FormsModule, ReactiveFormsModule } from '@angular/forms';
import { LoggerTestingModule } from 'ngx-logger/testing';
import { HttpClientTestingModule } from '@angular/common/http/testing';
import { ErrorService } from 'src/app/services/errors/error.service';
import { EncryptionService } from 'src/app/services/encryption/encryption.service';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatSelectModule } from '@angular/material/select';
import { MatTableModule } from '@angular/material/table';
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';

describe('ConfigureFeaturesModalComponent', () => {
  let component: ConfigureFeaturesModalComponent;
  let fixture: ComponentFixture<ConfigureFeaturesModalComponent>;

  let serverService: ServerService;
  let pluginService: PluginService;
  let formBuilder: FormBuilder;


  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ ConfigureFeaturesModalComponent ],
      imports: [LoggerTestingModule, HttpClientTestingModule, MatTableModule, BrowserAnimationsModule, MatFormFieldModule, MatSelectModule, ReactiveFormsModule, FormsModule],
      providers: [ErrorService, EncryptionService]
    })
    .compileComponents();

    fixture = TestBed.createComponent(ConfigureFeaturesModalComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();

    serverService = TestBed.inject(ServerService);
    pluginService = TestBed.inject(PluginService);
    formBuilder = TestBed.inject(FormBuilder);
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
