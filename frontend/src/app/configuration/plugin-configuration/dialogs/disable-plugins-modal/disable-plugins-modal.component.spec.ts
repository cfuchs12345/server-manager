import { ComponentFixture, TestBed } from '@angular/core/testing';

import { DisablePluginsModalComponent } from './disable-plugins-modal.component';

describe('ManagePluginsModalComponent', () => {
  let component: DisablePluginsModalComponent;
  let fixture: ComponentFixture<DisablePluginsModalComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ DisablePluginsModalComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(DisablePluginsModalComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
