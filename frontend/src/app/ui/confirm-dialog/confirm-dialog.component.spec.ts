import { ComponentFixture, TestBed } from '@angular/core/testing';

import { ConfirmDialogComponent } from './confirm-dialog.component';
import { MAT_DIALOG_DATA, MatDialogModule, MatDialogRef } from '@angular/material/dialog';
import { DomSanitizer } from '@angular/platform-browser';

describe('ConfirmDialogComponent', () => {
  let component: ConfirmDialogComponent;
  let fixture: ComponentFixture<ConfirmDialogComponent>;
  let sanitizer: DomSanitizer;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ ConfirmDialogComponent ],
      imports: [MatDialogModule],
      providers: [
        {
          provide: MatDialogRef,
          useValue: {}
        },
        {
          provide: MAT_DIALOG_DATA,
          useValue: {}
        },
      ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(ConfirmDialogComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();

    sanitizer = TestBed.inject(DomSanitizer);
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
