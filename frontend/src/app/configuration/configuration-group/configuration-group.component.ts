import { Component } from '@angular/core';
import { MatExpansionModule } from '@angular/material/expansion';

@Component({
    selector: 'app-configuration-group',
    templateUrl: './configuration-group.component.html',
    styleUrls: ['./configuration-group.component.scss'],
    standalone: true,
    imports: [MatExpansionModule],
})
export class ConfigurationGroupComponent {}
