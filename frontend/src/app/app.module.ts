import { NgModule, isDevMode } from '@angular/core';
import { AppComponent } from './app.component';
import { MatTooltipModule } from '@angular/material/tooltip';
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
import { MatTableModule } from '@angular/material/table';
import { MatProgressSpinnerModule } from '@angular/material/progress-spinner';
import { MatSelectModule } from '@angular/material/select';
import { HTTP_INTERCEPTORS, HttpClientModule } from '@angular/common/http';
import { LayoutModule } from '@angular/cdk/layout';
import { GridColsDirective } from './shared/grid-col-directive';
import { ActiveLightComponent } from './ui/active-light/active-light.component';
import { ServerListComponent } from './servers/server-list/server-list.component';
import { ServerIconComponent } from './servers/server-icon/server-icon.component';
import { ServerStatusComponent } from './servers/server-status/server-status.component';
import { ServerNotificationComponent } from './servers/server-notifications/server-notifications.component';
import { ServerActionListComponent } from './servers/server-action-list/server-action-list.component';
import { ServerActionComponent } from './servers/server-action/server-action.component';
import { ServerDetailComponent } from './servers/server-detail/server-detail.component';
import { ServerDetailControlComponent } from './servers/server-detail-control/server-detail-control.component';
import { ServerListWrapperComponent } from './servers/server-list-wrapper/server-list-wrapper.component';
import { AutodiscoverServerModalComponent } from './configuration/server-configuration/dialogs/autodiscover-server-modal/autodiscover-server-modal.component';
import { ConfigurationComponent } from './configuration/configuration/configuration.component';
import { AddServerModalComponent } from './configuration/server-configuration/dialogs/add-server-modal/add-server-modal.component';
import { ServerConfigurationComponent } from './configuration/server-configuration/server-configuration.component';
import { AutoDiscoveryDialogComponent } from './configuration/server-configuration/dialogs/dialog-autodiscover';
import { FeatureScanDialog } from './configuration/server-configuration/dialogs/dialog-feature-scan';
import { AddServerDialogComponent } from './configuration/server-configuration/dialogs/dialog-add-server';
import { DeleteServerDialogComponent } from './configuration/server-configuration/dialogs/dialog-delete-server';
import { ConfigurationGroupComponent } from './configuration/configuration-group/configuration-group.component';
import { GeneralConfigurationComponent } from './configuration/general-configuration/general-configuration.component';
import { FeatureScanModalComponent } from './configuration/server-configuration/dialogs/feature-scan-modal/feature-scan-modal.component';
import { ListPluginsDialogComponent } from './configuration/plugin-configuration/dialogs/dialog-listplugins';
import { DisablePluginsDialogComponent } from './configuration/plugin-configuration/dialogs/dialog-manageplugins';
import { ListPluginsModalComponent } from './configuration/plugin-configuration/dialogs/list-plugins-modal/list-plugins-modal.component';
import { DisablePluginsModalComponent } from './configuration/plugin-configuration/dialogs/disable-plugins-modal/disable-plugins-modal.component';
import { PluginConfigurationComponent } from './configuration/plugin-configuration/plugin-configuration.component';
import { ConfigureDnsModalComponent } from './configuration/general-configuration/dialogs/configure-dns-modal/configure-dns-modal.component';
import { ConfigureUsersModalComponent } from './configuration/general-configuration/dialogs/configure-users-modal/configure-users-modal.component';
import { ConfigureDNSDialogComponent } from './configuration/general-configuration/dialogs/dialog-configure-dns';
import { ConfigureUsersDialogComponent } from './configuration/general-configuration/dialogs/dialog-configure-users';
import { ChangePasswordDialogComponent } from './configuration/general-configuration/dialogs/dialog-change-password';
import { ConfigureFeaturesDialogComponent } from './configuration/server-configuration/dialogs/dialog-configure-features';
import { ConfigureFeaturesModalComponent } from './configuration/server-configuration/dialogs/configure-features-modal/configure-features-modal.component';
import { ChangePasswordModalComponent } from './configuration/general-configuration/dialogs/change-password-modal/change-password-modal.component';
import { ConfirmDialogComponent } from './ui/confirm-dialog/confirm-dialog.component';
import { MessageDialogComponent } from './ui/message_dialog/message-dialog.component';
import { LineChartComponent } from './ui/line-chart/line-chart.component';
import { BarChartComponent } from './ui/bar-chart/bar-chart.component';
import { ChartWrapperComponent } from './ui/chart-wrapper/chart-wrapper.component';
import { DeleteServerModalComponent } from './configuration/server-configuration/dialogs/delete-server-modal/delete-server-modal.component';
import { ErrorsListComponent } from './errors/errors-list/errors-list.component';
import { ErrorComponent } from './errors/error/error.component';
import { ErrorService } from './services/errors/error.service';
import { EncryptionService } from './services/encryption/encryption.service';
import { ImageCache } from './services/cache/image-cache.service';
import { ServerFeaturesComponent } from './servers/server-features/server-features.component';
import { ServerSubActionComponent } from './servers/server-sub-action/sub-action.component';
import { SystemInformationComponent } from './systeminformation/systeminformation.component';
import { MainComponent } from './main/main.component';
import { LoginComponent } from './login/login.component';
import { RegisterComponent } from './register/register.component';
import { TokenInterceptor } from './auth/token.interceptor';
import { ErrorInterceptor } from './errors/error.interceptor';
import { NgApexchartsModule } from 'ng-apexcharts';
import { MonitoringSingleServerComponent } from './monitoring/single-server/monitoring-single-server.component';
import { ConfigImExportDialogComponent } from './configuration/general-configuration/dialogs/dialog-config-im-and-export';
import { ConfigImExportModalComponent } from './configuration/general-configuration/dialogs/config-im-export-modal/config-im-export-modal.component';
import { ErrorSourceNamePipe } from './shared/error-enum-name.pipe';
import { LoggerModule, NgxLoggerLevel } from 'ngx-logger';
import { StoreModule } from '@ngrx/store';
import { StoreDevtoolsModule } from '@ngrx/store-devtools';
import { reducers, metaReducers } from './state';
import { ToastrModule } from 'ngx-toastr';
import { EffectsModule } from '@ngrx/effects';
import { HydrationEffects } from './state/hydration/hydration.effects';
import { GlobalEffects } from './state/global.effects';
import { ServerEffects } from './state/server/server.effects';
import { PluginEffects } from './state/plugin/plugin.effects';
import { DisabledPluginEffects } from './state/disabledplugin/disabled_plugin.effects';
import { UserEffects } from './state/user/user.effects';
import { StatusEffects } from './state/status/status.effects';
import { ConditionCheckResultEffects } from './state/conditioncheckresult/conditioncheckresult.effects';
import { ActionEffects } from './state/action/action.effects';

@NgModule({
    declarations: [AppComponent],
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
        MatSelectModule,
        MatTooltipModule,
        NgApexchartsModule,
        ToastrModule.forRoot(),
        LoggerModule.forRoot({
            level: NgxLoggerLevel.DEBUG,
        }),
        StoreModule.forRoot(reducers, {
            metaReducers,
        }),
        StoreDevtoolsModule.instrument({ maxAge: 25, logOnly: !isDevMode() }),
        EffectsModule.forRoot([
            HydrationEffects,
            GlobalEffects,
            ServerEffects,
            PluginEffects,
            DisabledPluginEffects,
            UserEffects,
            StatusEffects,
            ConditionCheckResultEffects,
            ActionEffects
        ]),
        ErrorSourceNamePipe,
        GridColsDirective,
        AutoDiscoveryDialogComponent,
        FeatureScanDialog,
        AddServerDialogComponent,
        DeleteServerDialogComponent,
        ListPluginsDialogComponent,
        DisablePluginsDialogComponent,
        ConfigureDNSDialogComponent,
        ConfigureUsersDialogComponent,
        ConfigureFeaturesDialogComponent,
        ChangePasswordDialogComponent,
        ActiveLightComponent,
        AutodiscoverServerModalComponent,
        ConfigurationComponent,
        ServerListComponent,
        ServerListWrapperComponent,
        ConfigImExportModalComponent,
        AddServerModalComponent,
        DeleteServerModalComponent,
        ServerConfigurationComponent,
        ServerNotificationComponent,
        ConfigurationGroupComponent,
        GeneralConfigurationComponent,
        FeatureScanModalComponent,
        PluginConfigurationComponent,
        ListPluginsModalComponent,
        DisablePluginsModalComponent,
        ConfigureDnsModalComponent,
        ConfigureUsersModalComponent,
        ConfigureFeaturesModalComponent,
        ChangePasswordModalComponent,
        ConfirmDialogComponent,
        MessageDialogComponent,
        ErrorsListComponent,
        ErrorComponent,
        ServerIconComponent,
        ServerStatusComponent,
        ServerActionComponent,
        ServerActionListComponent,
        ServerSubActionComponent,
        ServerDetailComponent,
        ServerDetailControlComponent,
        ServerFeaturesComponent,
        SystemInformationComponent,
        MainComponent,
        LoginComponent,
        RegisterComponent,
        LineChartComponent,
        BarChartComponent,
        ChartWrapperComponent,
        MonitoringSingleServerComponent,
        ConfigImExportDialogComponent,
    ],
    providers: [
        ErrorService,
        EncryptionService,
        ImageCache,
        { provide: Window, useValue: window },
        { provide: HTTP_INTERCEPTORS, useClass: TokenInterceptor, multi: true },
        { provide: HTTP_INTERCEPTORS, useClass: ErrorInterceptor, multi: true },
    ],
    bootstrap: [AppComponent],
})
export class AppModule {}
