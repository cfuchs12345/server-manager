import { NgModule } from '@angular/core';
import { AppComponent } from './app.component';
import { AppRoutingModule } from './app-routing.module';
import { BrowserModule } from '@angular/platform-browser';
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';
import { FlexLayoutModule } from '@angular/flex-layout';
import { FormsModule, ReactiveFormsModule } from '@angular/forms';
import { MatButtonModule } from '@angular/material/button';
import { MatCardModule } from '@angular/material/card';
import { MatDialogModule } from '@angular/material/dialog';
import { MatExpansionModule } from '@angular/material/expansion';
import { MatFormFieldModule } from '@angular/material/form-field';
import { MatGridListModule } from '@angular/material/grid-list';
import { MatInputModule } from '@angular/material/input';
import { MatTableModule } from '@angular/material/table'
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';
import { MatSelectModule } from '@angular/material/select';
import { HttpClientModule } from '@angular/common/http';
import { LayoutModule } from '@angular/cdk/layout';
import { GridColsDirective } from './shared/grid-col-directive';
import { ActiveLightComponent } from './ui/active-light/active-light.component';
import { ServerListComponent } from './servers/server-list/server-list.component';
import { ServerIconComponent } from './servers/server-icon/server-icon.component';
import { ServerStatusComponent } from './servers/server-status/server-status.component';
import { ServerActionListComponent } from './servers/server-action-list/server-action-list.component';
import { ServerActionComponent } from './servers/server-action/server-action.component';
import { ServerDetailComponent } from './servers/server-detail/server-detail.component';
import { ServerDetailControlComponent } from './servers/server-detail-control/server-detail-control.component';
import { ServerListWrapperComponent } from './servers/server-list-wrapper/server-list-wrapper.component';
import { AutodiscoverServerModalComponent } from './configuration/server-configuration/dialogs/autodiscover-server-modal/autodiscover-server-modal.component';
import { ConfigurationComponent } from './configuration/configuration/configuration.component';
import { AddServerModalComponent } from './configuration/server-configuration/dialogs/add-server-modal/add-server-modal.component';
import { ServerConfigurationComponent } from './configuration/server-configuration/server-configuration.component';
import { AutoDiscoveryDialog } from './configuration/server-configuration/dialogs/dialog-autodiscover';
import { FeatureScanDialog } from './configuration/server-configuration/dialogs/dialog-feature-scan';
import { AddServerDialog } from './configuration/server-configuration/dialogs/dialog-add-server';
import { DeleteServerDialog } from './configuration/server-configuration/dialogs/dialog-delete-server';
import { ConfigurationGroupComponent } from './configuration/configuration-group/configuration-group.component';
import { GeneralConfigurationComponent } from './configuration/general-configuration/general-configuration.component';
import { FeatureScanModalComponent } from './configuration/server-configuration/dialogs/feature-scan-modal/feature-scan-modal.component';
import { ListPluginsDialog } from './configuration/plugin-configuration/dialogs/dialog-listplugins';
import { DisablePluginsDialog } from './configuration/plugin-configuration/dialogs/dialog-manageplugins';
import { ListPluginsModalComponent } from './configuration/plugin-configuration/dialogs/list-plugins-modal/list-plugins-modal.component';
import { DisablePluginsModalComponent } from './configuration/plugin-configuration/dialogs/disable-plugins-modal/disable-plugins-modal.component';
import { PluginConfigurationComponent } from './configuration/plugin-configuration/plugin-configuration.component';
import { ConfigureDnsModalComponent } from './configuration/general-configuration/dialogs/configure-dns-modal/configure-dns-modal.component';
import { ConfigureDNSDialog } from './configuration/general-configuration/dialogs/dialog-configuredns';
import { ConfigureFeaturesModalComponent } from './configuration/server-configuration/dialogs/configure-features-modal/configure-features-modal.component';
import { ConfigureFeaturesDialog } from './configuration/server-configuration/dialogs/dialog-configure-features';
import { ConfirmDialogComponent } from './ui/confirm-dialog/confirm-dialog.component';
import { DeleteServerModalComponent } from './configuration/server-configuration/dialogs/delete-server-modal/delete-server-modal.component';
import { ErrorsListComponent } from './errors/errors-list/errors-list.component';
import { ErrorComponent } from './errors/error/error.component';
import { ErrorService } from './services/errors/error.service';
import { CacheService } from './services/cache/cache.service';
import { ImageCache } from './services/cache/image-cache.service';
import { ServerFeaturesComponent } from './servers/server-features/server-features.component';





@NgModule({
  declarations: [
    GridColsDirective,

    AutoDiscoveryDialog,
    FeatureScanDialog,
    AddServerDialog,
    DeleteServerDialog,
    ListPluginsDialog,
    DisablePluginsDialog,
    ConfigureDNSDialog,
    ConfigureFeaturesDialog,

    ActiveLightComponent,
    AutodiscoverServerModalComponent,
    AppComponent,
    ConfigurationComponent,
    ServerListComponent,
    ServerListWrapperComponent,
    AddServerModalComponent,
    DeleteServerModalComponent,
    ServerConfigurationComponent,
    ConfigurationGroupComponent,
    GeneralConfigurationComponent,
    FeatureScanModalComponent,
    PluginConfigurationComponent,
    ListPluginsModalComponent,
    DisablePluginsModalComponent,
    ConfigureDnsModalComponent,
    ConfigureFeaturesModalComponent,
    ConfirmDialogComponent,
    ErrorsListComponent,
    ErrorComponent,
    ServerIconComponent,
    ServerStatusComponent,
    ServerActionComponent,
    ServerActionListComponent,
    ServerDetailComponent,
    ServerDetailControlComponent,
    ServerFeaturesComponent
  ],
  imports: [
    AppRoutingModule,
    BrowserAnimationsModule,
    BrowserModule,
    FlexLayoutModule,
    FormsModule,
    HttpClientModule,
    MatCardModule,
    MatDialogModule,
    MatButtonModule,
    MatExpansionModule,
    MatFormFieldModule,
    MatGridListModule,
    MatProgressSpinnerModule,
    MatInputModule,
    MatTableModule,
    ReactiveFormsModule,
    LayoutModule,
    MatSelectModule
  ],
  providers: [ErrorService, CacheService, ImageCache],
  bootstrap: [AppComponent]
})
export class AppModule { }
