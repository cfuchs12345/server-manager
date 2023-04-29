import { ComponentFixture, TestBed } from '@angular/core/testing';

import { ActiveLightComponent } from './active-light.component';

describe('AppActiveLightComponent', () => {
  let component: ActiveLightComponent;
  let fixture: ComponentFixture<ActiveLightComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ ActiveLightComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(ActiveLightComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
