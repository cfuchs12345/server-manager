import { ComponentFixture, TestBed } from '@angular/core/testing';

import { FeatureScanModalComponent } from './feature-scan-modal.component';

describe('FeatureScanModalComponent', () => {
  let component: FeatureScanModalComponent;
  let fixture: ComponentFixture<FeatureScanModalComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ FeatureScanModalComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(FeatureScanModalComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
