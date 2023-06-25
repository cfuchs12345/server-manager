import { ComponentFixture, TestBed } from '@angular/core/testing';

import { ErrorComponent } from './error.component';
import { ErrorService } from 'src/app/services/errors/error.service';

describe('ErrorComponent', () => {
  let component: ErrorComponent;
  let fixture: ComponentFixture<ErrorComponent>;
  let errorService: ErrorService;
  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ ErrorComponent ],
      providers: [ErrorService]
    })
    .compileComponents();

    fixture = TestBed.createComponent(ErrorComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();

    errorService = TestBed.inject(ErrorService);
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
