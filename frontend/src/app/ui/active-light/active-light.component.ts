import { Component, Input } from '@angular/core';

@Component({
  selector: 'app-active-light',
  templateUrl: './active-light.component.html',
  styleUrls: ['./active-light.component.scss'],
})
export class ActiveLightComponent {
  @Input() isActive: boolean = false;

  constructor() {}
}
