import { ComponentFixture, TestBed } from '@angular/core/testing';

import { ListPluginsModalComponent } from './list-plugins-modal.component';

describe('ListPluginsModalComponent', () => {
  let component: ListPluginsModalComponent;
  let fixture: ComponentFixture<ListPluginsModalComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ ListPluginsModalComponent ]
    })
    .compileComponents();

    fixture = TestBed.createComponent(ListPluginsModalComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
