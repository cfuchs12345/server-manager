import { Component, Input } from '@angular/core';
import { ExtendedModule } from '@angular/flex-layout/extended';
import { NgClass } from '@angular/common';

@Component({
    selector: 'app-active-light',
    templateUrl: './active-light.component.html',
    styleUrls: ['./active-light.component.scss'],
    standalone: true,
    imports: [NgClass, ExtendedModule],
})
export class ActiveLightComponent {
  @Input() isActive: boolean | null = false;

}
