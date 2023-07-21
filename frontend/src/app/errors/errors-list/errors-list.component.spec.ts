import { ComponentFixture, TestBed } from '@angular/core/testing';

import { ErrorsListComponent } from './errors-list.component';
import { ErrorService } from 'src/app/services/errors/error.service';
import { ConfigurationGroupComponent } from 'src/app/configuration/configuration-group/configuration-group.component';
import { ErrorComponent } from '../error/error.component';
import { MatExpansionModule } from '@angular/material/expansion';
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';
import { LoggerTestingModule, NGXLoggerMock } from 'ngx-logger/testing';

describe('ErrorsListComponent', () => {
  let component: ErrorsListComponent;
  let fixture: ComponentFixture<ErrorsListComponent>;
  // eslint-disable-next-line   @typescript-eslint/no-unused-vars
  let errorService: ErrorService;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [
        MatExpansionModule,
        BrowserAnimationsModule,
        ErrorsListComponent,
        ConfigurationGroupComponent,
        ErrorComponent,
        LoggerTestingModule,
      ],
      providers: [ErrorService, NGXLoggerMock],
    }).compileComponents();

    fixture = TestBed.createComponent(ErrorsListComponent);
    component = fixture.componentInstance;
    // eslint-disable-next-line  @rx-angular/no-explicit-change-detection-apis
    fixture.detectChanges();

    errorService = TestBed.inject(ErrorService);
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
