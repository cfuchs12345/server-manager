import { HostListener, Component, OnInit, Input, OnDestroy  } from '@angular/core';
import { DomSanitizer, SafeHtml } from '@angular/platform-browser';
import { Subscription } from 'rxjs';
import { CacheService } from 'src/app/services/cache/cache.service';
import { ImageCache } from 'src/app/services/cache/image-cache.service';
import { PluginService } from 'src/app/services/plugins/plugin.service';
import { Plugin } from 'src/app/services/plugins/types';
import { Feature, Server, Status } from 'src/app/services/servers/types';




@Component({
  selector: 'app-server-icon',
  templateUrl: './server-icon.component.html',
  styleUrls: ['./server-icon.component.scss'],
})
export class ServerIconComponent implements OnInit, OnDestroy {
  @Input() server: Server | undefined = undefined;

  constructor(private imageCache: ImageCache) {
  }

  ngOnInit(): void {
  }

  ngOnDestroy(): void {
  }


  getServerIcon = (): SafeHtml | undefined => {
    if( !this.server ) {
      return undefined;
    }

    const icon = this.findFeatureIcon(this.server);
    if( icon ) {
      return icon;
    }
    return this.imageCache.getDefaultIcon();
  }



  private findFeatureIcon = (server: Server): SafeHtml | undefined  => {
    if (server.features) {
      for( const feature of server.features ) {
        const icon = this.imageCache.getImageForFeature(feature.id);

        if( icon ) {
          return icon;
        }
      }
    }
    return undefined;
  }
}
