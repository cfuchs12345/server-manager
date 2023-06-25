import { ComponentFixture, TestBed } from '@angular/core/testing';

import { DisablePluginsModalComponent } from './disable-plugins-modal.component';
import { HttpClientTestingModule } from '@angular/common/http/testing';
import { PluginService } from 'src/app/services/plugins/plugin.service';
import { ErrorService } from 'src/app/services/errors/error.service';
import { MatTableModule } from '@angular/material/table';

describe('DisablePluginsModalComponent', () => {
  let component: DisablePluginsModalComponent;
  let fixture: ComponentFixture<DisablePluginsModalComponent>;

  let pluginService: PluginService;
  let errorService: ErrorService;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [HttpClientTestingModule, MatTableModule],
      providers: [ErrorService],
      declarations: [ DisablePluginsModalComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(DisablePluginsModalComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();

    pluginService = TestBed.inject(PluginService);
    errorService = TestBed.inject(ErrorService);
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
