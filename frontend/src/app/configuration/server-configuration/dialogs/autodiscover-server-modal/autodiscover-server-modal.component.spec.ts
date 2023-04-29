import { ComponentFixture, TestBed } from '@angular/core/testing';

import { AutodiscoverServerModalComponent } from './autodiscover-server-modal.component';

describe('AutodiscoverServerModalComponent', () => {
  let component: AutodiscoverServerModalComponent;
  let fixture: ComponentFixture<AutodiscoverServerModalComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ AutodiscoverServerModalComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(AutodiscoverServerModalComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
