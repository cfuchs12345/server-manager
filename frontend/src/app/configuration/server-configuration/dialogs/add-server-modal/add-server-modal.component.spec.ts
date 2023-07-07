import { ComponentFixture, TestBed } from '@angular/core/testing';

import { AddServerModalComponent } from './add-server-modal.component';
import { ServerService } from 'src/app/services/servers/server.service';
import { PluginService } from 'src/app/services/plugins/plugin.service';
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

describe('AddServerModalComponent', () => {
  let component: AddServerModalComponent;
  let fixture: ComponentFixture<AddServerModalComponent>;
    // eslint-disable-next-line  @typescript-eslint/no-unused-vars
  let serverService: ServerService;
    // eslint-disable-next-line  @typescript-eslint/no-unused-vars
  let pluginService: PluginService;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ AddServerModalComponent ],
      imports: [LoggerTestingModule, HttpClientTestingModule, MatFormFieldModule, MatTableModule, BrowserAnimationsModule, MatSelectModule, MatInputModule, ReactiveFormsModule, FormsModule ],
      providers: [ErrorService, EncryptionService]
    })
    .compileComponents();

    fixture = TestBed.createComponent(AddServerModalComponent);
    component = fixture.componentInstance;

    // eslint-disable-next-line  @rx-angular/no-explicit-change-detection-apis
    fixture.detectChanges();

    serverService = TestBed.inject(ServerService);
    pluginService = TestBed.inject(PluginService);
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
