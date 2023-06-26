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

  it('should be green', () => {
    component.isActive = true;
    fixture.detectChanges();

    let span: HTMLElement = fixture.nativeElement.querySelector('span');

    expect(span.className).toContain('active')
    expect(span.className).not.toContain('inactive');
  })

  it('should be red', () => {
    component.isActive = false;
    fixture.detectChanges();

    let span: HTMLElement = fixture.nativeElement.querySelector('span');

    expect(span.className).toContain('inactive')
  })
});
