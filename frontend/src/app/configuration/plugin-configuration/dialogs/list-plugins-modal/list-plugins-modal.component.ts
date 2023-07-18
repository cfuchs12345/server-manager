import { Component, OnDestroy, OnInit, inject } from '@angular/core';
import { Plugin } from '../../../../services/plugins/types';
import { Observable, of } from 'rxjs';
import { Store } from '@ngrx/store';
import { selectAllPlugins } from 'src/app/state/plugin/plugin.selectors';
import { SubscriptionHandler } from 'src/app/shared/subscriptionHandler';

@Component({
  selector: 'app-list-plugins-modal',
  templateUrl: './list-plugins-modal.component.html',
  styleUrls: ['./list-plugins-modal.component.scss'],
})
export class ListPluginsModalComponent implements OnInit, OnDestroy {
  private store = inject(Store);
  private subscriptionHandler = new SubscriptionHandler(this);

  plugins$: Observable<Plugin[]> = of([]);

  displayedColumns: string[] = ['description', 'detection'];

  ngOnInit(): void {
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
