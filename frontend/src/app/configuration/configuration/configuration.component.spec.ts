import { ComponentFixture, TestBed } from '@angular/core/testing';

import { ConfigurationFormComponent } from './configuration.component';

describe('ConfigurationFormComponent', () => {
  let component: ConfigurationFormComponent;
  let fixture: ComponentFixture<ConfigurationFormComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ ConfigurationFormComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(ConfigurationFormComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
