import { ComponentFixture, TestBed } from '@angular/core/testing';
import { ConfigurationGroupComponent } from './configuration-group.component';
import { MatExpansionModule } from '@angular/material/expansion';
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';

describe('ConfigurationGroupComponent', () => {
  let component: ConfigurationGroupComponent;
  let fixture: ComponentFixture<ConfigurationGroupComponent>;

  beforeEach(async () => {
    await TestBed.configureTestingModule({
      declarations: [ConfigurationGroupComponent],
      imports: [MatExpansionModule, BrowserAnimationsModule]
    }).compileComponents();

    fixture = TestBed.createComponent(ConfigurationGroupComponent);
    component = fixture.componentInstance;
    fixture.detectChanges();
  });

  it('should create', () => {
    expect(component).toBeTruthy();
  });
});
