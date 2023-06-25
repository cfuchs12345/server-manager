import { ComponentFixture, TestBed } from '@angular/core/testing';

import { GeneralConfigurationComponent } from './general-configuration.component';
import { MAT_DIALOG_DATA, MatDialog } from '@angular/material/dialog';
import { ConfigurationGroupComponent } from '../configuration-group/configuration-group.component';
import { MatExpansionModule } from '@angular/material/expansion';
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';

describe('GeneralConfigurationComponent', () => {
  let component: GeneralConfigurationComponent;
  let fixture: ComponentFixture<GeneralConfigurationComponent>;
  let matDialog: MatDialog;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ GeneralConfigurationComponent, ConfigurationGroupComponent ],
      imports: [MatExpansionModule, BrowserAnimationsModule],
      providers: [{
        provide: MatDialog,
        useValue: {}
      },
      { provide: MAT_DIALOG_DATA, useValue: {} }
    ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(GeneralConfigurationComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();

    matDialog = TestBed.inject(MatDialog);
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
