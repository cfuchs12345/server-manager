import { ComponentFixture, TestBed } from '@angular/core/testing';

import { ConfigurationComponent } from './configuration.component';
import { ConfigurationGroupComponent } from '../configuration-group/configuration-group.component';
import { ServerConfigurationComponent } from '../server-configuration/server-configuration.component';
import { MatDialog } from '@angular/material/dialog';
import { PluginConfigurationComponent } from '../plugin-configuration/plugin-configuration.component';
import { GeneralConfigurationComponent } from '../general-configuration/general-configuration.component';
import { MatExpansionModule } from '@angular/material/expansion';
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';

describe('ConfigurationComponent', () => {
  let component: ConfigurationComponent;
  let fixture: ComponentFixture<ConfigurationComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ConfigurationComponent, ConfigurationGroupComponent, ServerConfigurationComponent, PluginConfigurationComponent, GeneralConfigurationComponent],
      imports: [MatExpansionModule, BrowserAnimationsModule],
      providers: [
        {
          provide: MatDialog,
          useValue: {}
        }
      ]
    }).compileComponents();

    fixture = TestBed.createComponent(ConfigurationComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
