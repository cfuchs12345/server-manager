import { ComponentFixture, TestBed } from '@angular/core/testing';

import { PluginConfigurationComponent } from './plugin-configuration.component';
import { MatDialog } from '@angular/material/dialog';
import { ConfigurationGroupComponent } from '../configuration-group/configuration-group.component';
import { MatExpansionModule } from '@angular/material/expansion';
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';

describe('PluginConfigurationComponent', () => {
  let component: PluginConfigurationComponent;
  let fixture: ComponentFixture<PluginConfigurationComponent>;

  // eslint-disable-next-line  @typescript-eslint/no-unused-vars
  let matDialog: MatDialog;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ PluginConfigurationComponent, ConfigurationGroupComponent ],
      imports: [MatExpansionModule, BrowserAnimationsModule],
      providers: [{
        provide: MatDialog,
        useValue: {}
      },]
    })
    .compileComponents();

    fixture = TestBed.createComponent(PluginConfigurationComponent);
    component = fixture.componentInstance;

    // eslint-disable-next-line  @rx-angular/no-explicit-change-detection-apis
    fixture.detectChanges();

    matDialog = TestBed.inject(MatDialog);
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
