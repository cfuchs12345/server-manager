import { DomSanitizer, SafeHtml } from '@angular/platform-browser';
import { Plugin } from '../plugins/types';
import { Injectable } from '@angular/core';

const defaultSererIcon: string = `<svg fill="#000000" version="1.1" id="XMLID_131_" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 24 24" xml:space="preserve" width="25px" height="25px">
<g id="SVGRepo_bgCarrier" stroke-width="0"/>
<g id="SVGRepo_tracerCarrier" stroke-linecap="round" stroke-linejoin="round"/>
<g id="SVGRepo_iconCarrier"> <g id="server"> <g> <path d="M20,24H4V0h16V24z M6,22h12V2H6V22z M16,14H8v-2h8V14z M16,10H8V8h8V10z M16,6H8V4h8V6z"/> </g> <g> <rect x="14" y="18" width="2" height="2"/> </g> </g> </g>
</svg>`;

@Injectable()
export class ImageCache {
  private defaultIcon: SafeHtml;
  private featureIcons: Map<string, SafeHtml> = new Map();
  private actionIcons: Map<string, Map<string, SafeHtml>> = new Map();

  constructor(private sanitizer: DomSanitizer) {
    this.defaultIcon = this.sanitizer.bypassSecurityTrustHtml(defaultSererIcon);
  }

  public init(plugins: Plugin[]) {
    this.featureIcons = this.createServerIconMap(plugins);
    this.actionIcons = this.createActionIconMap(plugins);
  }

  getDefaultIcon(): SafeHtml | undefined {
    return this.defaultIcon;
  }
  getImageForFeature(feature_id: string): SafeHtml | undefined {
    return this.featureIcons.get(feature_id);
  }

  getImageFeatureAction(
    feature_id: string,
    action_id: string
  ): SafeHtml | undefined {
    const actionsMap = this.actionIcons.get(feature_id);
    if (actionsMap) {
      return actionsMap.get(action_id);
    }
    return undefined;
  }

  private createServerIconMap = (plugins: Plugin[]): Map<string, SafeHtml> => {
    const map: Map<string, SafeHtml> = new Map();

    if (plugins) {
      plugins.forEach((plugin) => {
        if (plugin.server_icon) {
          map.set(
            plugin.id,
            this.sanitizer.bypassSecurityTrustHtml(plugin.server_icon)
          );
        }
      });
    }
    return map;
  };

  private createActionIconMap = (
    plugins: Plugin[]
  ): Map<string, Map<string, SafeHtml>> => {
    const map: Map<string, Map<string, SafeHtml>> = new Map();

    if (plugins) {
      plugins.forEach((plugin) => {
        const actionMap: Map<string, SafeHtml> = new Map();

        if (plugin.actions && Array.isArray(plugin.actions)) {
          plugin.actions.forEach((action) => {
            actionMap.set(
              action.id,
              this.sanitizer.bypassSecurityTrustHtml(action.icon)
            );
          });

          map.set(plugin.id, actionMap);
        } else {
          alert(JSON.stringify(plugin.actions));
        }
      });
    }
    return map;
  };
}
