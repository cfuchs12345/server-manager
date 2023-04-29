import { ComponentFixture, TestBed } from '@angular/core/testing';

import { ConfigureFeaturesModalComponent } from './configure-features-modal.component';

describe('ConfigureFeaturesComponent', () => {
  let component: ConfigureFeaturesModalComponent;
  let fixture: ComponentFixture<ConfigureFeaturesModalComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ ConfigureFeaturesModalComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(ConfigureFeaturesModalComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
