import { ComponentFixture, TestBed } from '@angular/core/testing';

import { PluginConfigurationComponent } from './plugin-configuration.component';

describe('PluginConfigurationComponent', () => {
  let component: PluginConfigurationComponent;
  let fixture: ComponentFixture<PluginConfigurationComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ PluginConfigurationComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(PluginConfigurationComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
