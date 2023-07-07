import { Component, OnDestroy } from '@angular/core';
import { Plugin } from '../../../../services/plugins/types';
import { Observable } from 'rxjs';
import { Store } from '@ngrx/store';
import { selectAllPlugins } from 'src/app/state/plugin/plugin.selectors';
import { SubscriptionHandler } from 'src/app/shared/subscriptionHandler';

@Component({
  selector: 'app-list-plugins-modal',
  templateUrl: './list-plugins-modal.component.html',
  styleUrls: ['./list-plugins-modal.component.scss'],
})
export class ListPluginsModalComponent implements  OnDestroy {
  displayedColumns: string[] = ['description', 'detection'];

  readonly plugins$: Observable<Plugin[]>;

  private subscriptionHandler = new SubscriptionHandler(this);

  constructor(private store: Store) {
    this.plugins$ = this.store.select(selectAllPlugins);
  }

  ngOnDestroy(): void {
    this.subscriptionHandler.onDestroy();
  }

  detectionPossible(plugin: Plugin): boolean {
    return (
      plugin &&
      plugin.detection &&
      plugin.detection.detection_possible &&
      plugin.detection.detection_possible === true
    );
  }
}
