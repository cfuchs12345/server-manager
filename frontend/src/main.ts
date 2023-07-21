import { bootstrapApplication } from '@angular/platform-browser';
import { AppComponent } from './app/app.component';
import { ActionEffects } from './app/state/action/action.effects';
import { ConditionCheckResultEffects } from './app/state/conditioncheckresult/conditioncheckresult.effects';
import { StatusEffects } from './app/state/status/status.effects';
import { UserEffects } from './app/state/user/user.effects';
import { DisabledPluginEffects } from './app/state/disabledplugin/disabled_plugin.effects';
import { PluginEffects } from './app/state/plugin/plugin.effects';
import { ServerEffects } from './app/state/server/server.effects';
import { GlobalEffects } from './app/state/global.effects';
import { HydrationEffects } from './app/state/hydration/hydration.effects';
import { EffectsModule } from '@ngrx/effects';
import { isDevMode, importProvidersFrom } from '@angular/core';
import { StoreDevtoolsModule } from '@ngrx/store-devtools';
import { reducers, metaReducers } from './app/state';
import { StoreModule } from '@ngrx/store';
import { LoggerModule, NgxLoggerLevel } from 'ngx-logger';
import { ToastrModule } from 'ngx-toastr';
import { APP_ROUTES } from './app/app-routing.module';
import { errorInterceptor } from './app/errors/error.interceptor';
import { tokenInterceptor } from './app/auth/token.interceptor';
import { withInterceptors  } from '@angular/common/http';
import { ImageCache } from './app/services/cache/image-cache.service';
import { EncryptionService } from './app/services/encryption/encryption.service';
import { ErrorService } from './app/services/errors/error.service';
import { provideHttpClient } from '@angular/common/http';
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';
import { BrowserModule } from '@angular/platform-browser';
import { MatDialogModule } from '@angular/material/dialog';
import { RouterModule } from '@angular/router';

declare global {
  // eslint-disable-next-line  @typescript-eslint/no-explicit-any
  interface Window {
    MyServerManagerNS: any;
  }
}
window.MyServerManagerNS = window.MyServerManagerNS || {};

bootstrapApplication(AppComponent, {
  providers: [
    provideHttpClient(withInterceptors([errorInterceptor, tokenInterceptor])),
    importProvidersFrom(
      MatDialogModule,
      BrowserModule,
      BrowserAnimationsModule,
      RouterModule.forRoot(APP_ROUTES),
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
        ActionEffects,
      ])
    ),
    ErrorService,
    EncryptionService,
    ImageCache,
    { provide: Window, useValue: window },
  ],
}).catch((err) => console.error(err));
