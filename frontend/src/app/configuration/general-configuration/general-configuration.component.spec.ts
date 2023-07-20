import { ComponentFixture, TestBed } from '@angular/core/testing';

import { GeneralConfigurationComponent } from './general-configuration.component';
import { MAT_DIALOG_DATA, MatDialog } from '@angular/material/dialog';
import { ConfigurationGroupComponent } from '../configuration-group/configuration-group.component';
import { MatExpansionModule } from '@angular/material/expansion';
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';

describe('GeneralConfigurationComponent', () => {
  let component: GeneralConfigurationComponent;
  let fixture: ComponentFixture<GeneralConfigurationComponent>;

  // eslint-disable-next-line  @typescript-eslint/no-unused-vars
  let matDialog: MatDialog;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
    imports: [MatExpansionModule, BrowserAnimationsModule, GeneralConfigurationComponent, ConfigurationGroupComponent],
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

    // eslint-disable-next-line  @rx-angular/no-explicit-change-detection-apis
    fixture.detectChanges();

    matDialog = TestBed.inject(MatDialog);
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
