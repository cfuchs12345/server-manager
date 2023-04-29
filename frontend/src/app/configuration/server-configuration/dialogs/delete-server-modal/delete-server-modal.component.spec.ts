import { ComponentFixture, TestBed } from '@angular/core/testing';

import { DeleteServerModalComponent } from './delete-server-modal.component';

describe('DeleteServerModalComponent', () => {
  let component: DeleteServerModalComponent;
  let fixture: ComponentFixture<DeleteServerModalComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ DeleteServerModalComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(DeleteServerModalComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
