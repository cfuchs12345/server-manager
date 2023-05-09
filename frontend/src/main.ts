import { platformBrowserDynamic } from '@angular/platform-browser-dynamic';

import { AppModule } from './app/app.module';

declare global {
  interface Window { MyServerManagerNS: any; }
}
window.MyServerManagerNS = window.MyServerManagerNS || {};


platformBrowserDynamic().bootstrapModule(AppModule)
  .catch(err => console.error(err));
