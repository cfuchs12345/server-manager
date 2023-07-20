import { ComponentFixture, TestBed } from '@angular/core/testing';

import { ErrorComponent } from './error.component';
import { ErrorService } from 'src/app/services/errors/error.service';

describe('ErrorComponent', () => {
  let component: ErrorComponent;
  let fixture: ComponentFixture<ErrorComponent>;
  // eslint-disable-next-line  @typescript-eslint/no-unused-vars
  let errorService: ErrorService;
  beforeEach(async () => {
    await TestBed.configureTestingModule({
    imports: [ErrorComponent],
    providers: [ErrorService]
})
    .compileComponents();

    fixture = TestBed.createComponent(ErrorComponent);
    component = fixture.componentInstance;
    // eslint-disable-next-line  @rx-angular/no-explicit-change-detection-apis
    fixture.detectChanges();

    errorService = TestBed.inject(ErrorService);
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
