import { Component, Input, OnInit, inject } from '@angular/core';
import { Store } from '@ngrx/store';
import { Observable, filter, map } from 'rxjs';
import { ErrorService, Source } from 'src/app/services/errors/error.service';
import { Plugin } from 'src/app/services/plugins/types';
import { Server } from 'src/app/services/servers/types';
import { selectAllPlugins } from 'src/app/state/plugin/plugin.selectors';

@Component({
  selector: 'app-server-features',
  templateUrl: './server-features.component.html',
  styleUrls: ['./server-features.component.scss'],
})
export class ServerFeaturesComponent implements OnInit {
  private store = inject(Store);
  private errorService = inject(ErrorService);

  @Input() server: Server | undefined = undefined;

  features$?: Observable<string[]>;

  private plugins$?: Observable<Plugin[]>;

  ngOnInit() {
    this.plugins$ = this.store
      .select(selectAllPlugins)
      .pipe(filter((plugins) => this.filter(plugins)));

    this.features$ = this.plugins$.pipe(
      map((plugins) => this.getFeatures(plugins))
    );
  }

  private getFeatures = (plugins: Plugin[]): string[] => {
    if (
      !this.server ||
      !this.server.features ||
      this.server.features.length === 0
    ) {
      return [];
    }

    const plugin_names: string[] = [];
    for (const feature of this.server.features) {
      const plugin = plugins.find((p) => p.id === feature.id);
      if (plugin) {
        plugin_names.push(plugin.name);
      } else {
        this.errorService.newError(
          Source.ServerFeaturesComponent,
          this.server.ipaddress,
          'Plugin for feature ' + feature.name + ' not found.'
        );
      }
    }
    return plugin_names.sort();
  };

  private filter(plugins: Plugin[]): boolean {
    return plugins.find((p) => this.isPluginRequired(p)) !== undefined;
  }
  private isPluginRequired(plugin: Plugin): boolean {
    return (
      this.server !== undefined &&
      this.server.features.find((f) => f.id === plugin.id) !== undefined
    );
  }
}
