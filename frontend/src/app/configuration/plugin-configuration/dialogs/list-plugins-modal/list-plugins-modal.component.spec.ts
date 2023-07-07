import { ComponentFixture, TestBed } from '@angular/core/testing';
import { HttpClientTestingModule}  from '@angular/common/http/testing';
import { ListPluginsModalComponent } from './list-plugins-modal.component';
import { PluginService } from 'src/app/services/plugins/plugin.service';
import { ErrorService } from 'src/app/services/errors/error.service';
import { MatTableModule } from '@angular/material/table';

describe('ListPluginsModalComponent', () => {
  let component: ListPluginsModalComponent;
  let fixture: ComponentFixture<ListPluginsModalComponent>;
  // eslint-disable-next-line  @typescript-eslint/no-unused-vars
  let servicePlugins: PluginService;
  // eslint-disable-next-line  @typescript-eslint/no-unused-vars
  let errorService: ErrorService;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      imports: [HttpClientTestingModule, MatTableModule],
      declarations: [ ListPluginsModalComponent ],
      providers: [ErrorService]
    })
    .compileComponents();

    fixture = TestBed.createComponent(ListPluginsModalComponent);
    component = fixture.componentInstance;

    // eslint-disable-next-line  @rx-angular/no-explicit-change-detection-apis
    fixture.detectChanges();

    servicePlugins = TestBed.inject(PluginService);
    errorService = TestBed.inject(ErrorService);
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
