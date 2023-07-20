import { ComponentFixture, TestBed } from '@angular/core/testing';

import { ServerConfigurationComponent } from './server-configuration.component';
import { MAT_DIALOG_DATA, MatDialog } from '@angular/material/dialog';
import { ConfigurationGroupComponent } from '../configuration-group/configuration-group.component';
import { MatExpansionModule } from '@angular/material/expansion';
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';

describe('ServerConfigurationComponent', () => {
  let component: ServerConfigurationComponent;
  let fixture: ComponentFixture<ServerConfigurationComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
    imports: [MatExpansionModule, BrowserAnimationsModule, ServerConfigurationComponent, ConfigurationGroupComponent],
    providers: [{
            provide: MatDialog,
            useValue: {}
        },
        { provide: MAT_DIALOG_DATA, useValue: {} }
    ]
})

    .compileComponents();

    fixture = TestBed.createComponent(ServerConfigurationComponent);
    component = fixture.componentInstance;

    // eslint-disable-next-line  @rx-angular/no-explicit-change-detection-apis
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
