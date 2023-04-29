import { ComponentFixture, TestBed } from '@angular/core/testing';

import { AddServerModalComponent } from './add-server-modal.component';

describe('ManageManuallyModalComponent', () => {
  let component: AddServerModalComponent;
  let fixture: ComponentFixture<AddServerModalComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ AddServerModalComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(AddServerModalComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
