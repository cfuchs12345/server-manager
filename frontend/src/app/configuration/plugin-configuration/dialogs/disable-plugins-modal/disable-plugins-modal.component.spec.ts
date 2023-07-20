import { ComponentFixture, TestBed } from '@angular/core/testing';

import { DisablePluginsModalComponent } from './disable-plugins-modal.component';
import { HttpClientTestingModule } from '@angular/common/http/testing';
import { PluginService } from 'src/app/services/plugins/plugin.service';
import { ErrorService } from 'src/app/services/errors/error.service';
import { MatTableModule } from '@angular/material/table';

describe('DisablePluginsModalComponent', () => {
  let component: DisablePluginsModalComponent;
  let fixture: ComponentFixture<DisablePluginsModalComponent>;

  // eslint-disable-next-line  @typescript-eslint/no-unused-vars
  let pluginService: PluginService;
  // eslint-disable-next-line  @typescript-eslint/no-unused-vars
  let errorService: ErrorService;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
    imports: [HttpClientTestingModule, MatTableModule, DisablePluginsModalComponent],
    providers: [ErrorService]
})
    .compileComponents();

    fixture = TestBed.createComponent(DisablePluginsModalComponent);
    component = fixture.componentInstance;

    // eslint-disable-next-line  @rx-angular/no-explicit-change-detection-apis
    fixture.detectChanges();

    pluginService = TestBed.inject(PluginService);
    errorService = TestBed.inject(ErrorService);
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
