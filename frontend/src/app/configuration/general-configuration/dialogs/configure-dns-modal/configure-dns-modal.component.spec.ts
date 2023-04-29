import { ComponentFixture, TestBed } from '@angular/core/testing';

import { ConfigureDnsModalComponent } from './configure-dns-modal.component';

describe('ManageDnsModalComponent', () => {
  let component: ConfigureDnsModalComponent;
  let fixture: ComponentFixture<ConfigureDnsModalComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ ConfigureDnsModalComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(ConfigureDnsModalComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
