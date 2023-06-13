import { Component, Input, OnChanges, OnDestroy, OnInit } from '@angular/core';
import { Subscription, filter } from 'rxjs';
import { ErrorService, Source } from 'src/app/services/errors/error.service';
import { PluginService } from 'src/app/services/plugins/plugin.service';
import { Plugin } from 'src/app/services/plugins/types';
import { Server } from 'src/app/services/servers/types';

@Component({
  selector: 'app-server-features',
  templateUrl: './server-features.component.html',
  styleUrls: ['./server-features.component.scss'],
})
export class ServerFeaturesComponent implements OnInit, OnDestroy, OnChanges {
  @Input() server: Server | undefined = undefined;

  features: string[] | undefined = undefined;

  private plugins: Plugin[] | undefined = undefined;
  private pluginSubscription: Subscription | undefined = undefined;

  constructor(
    private pluginService: PluginService,
    private errorService: ErrorService
  ) {}

  ngOnInit(): void {
    this.pluginSubscription = this.pluginService.plugins
      .pipe(filter((plugins) => this.filter(plugins)))
      .subscribe((plugins) => (this.plugins = plugins));

    this.features = this.getFeatures();
  }

  ngOnChanges(): void {
    this.features = this.getFeatures();
  }

  ngOnDestroy(): void {
    if (this.pluginSubscription) {
      this.pluginSubscription.unsubscribe();
    }
  }

  private getFeatures = (): string[] => {
    if (
      !this.server ||
      !this.plugins ||
      !this.server.features ||
      this.server.features.length === 0
    ) {
      return [];
    }

    var plugin_names: string[] = [];
    for (var feature of this.server.features) {
      var plugin = this.plugins.find((p) => p.id === feature.id);
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
