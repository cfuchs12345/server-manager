import { Component } from '@angular/core';
import { ServerListWrapperComponent } from '../servers/server-list-wrapper/server-list-wrapper.component';
import { ServerSubActionComponent } from '../servers/server-sub-action/sub-action.component';
import { ErrorsListComponent } from '../errors/errors-list/errors-list.component';
import { ConfigurationComponent } from '../configuration/configuration/configuration.component';
import { SystemInformationComponent } from '../systeminformation/systeminformation.component';

@Component({
    selector: 'app-main',
    templateUrl: './main.component.html',
    styleUrls: ['./main.component.scss'],
    standalone: true,
    imports: [SystemInformationComponent, ConfigurationComponent, ErrorsListComponent, ServerSubActionComponent, ServerListWrapperComponent]
})
export class MainComponent {
  title = 'My Homelab Server Manager';
}
