import { Component, Input, OnDestroy, OnChanges } from '@angular/core';
import { DomSanitizer, SafeHtml } from '@angular/platform-browser';
import { Subscription } from 'rxjs';
import { ServerDataService } from 'src/app/services/servers/server-data.service';
import { DataResult, Server } from 'src/app/services/servers/types';

const action_regex = /\[\[Action.*\]\]/g;

@Component({
  selector: 'app-server-detail',
  templateUrl: './server-detail.component.html',
  styleUrls: ['./server-detail.component.scss'],
})
export class ServerDetailComponent implements OnChanges, OnDestroy {
  @Input() server: Server | undefined = undefined;
  @Input() showDetail: boolean = false;
  @Input() turnDetail: boolean = false;

  dataResults: Map<String, DataResult> = new Map();
  dataResultSubscription: Subscription | undefined = undefined;

  constructor(
    private sanitizer: DomSanitizer,
    private serverDataService: ServerDataService
  ) {}

  ngOnChanges(): void {
    if (this.showDetail) {
      this.dataResultSubscription =
        this.serverDataService.dataResults.subscribe((result) => {
          this.dataResults = result;
        });

      if (this.server) {
        this.serverDataService.queryData(this.server);
      }
    } else {
      if (this.dataResultSubscription) {
        this.dataResultSubscription.unsubscribe();
      }
    }
  }

  ngOnDestroy(): void {
    if (this.dataResultSubscription) {
      this.dataResultSubscription.unsubscribe();
    }
  }

  getDataResult = (): SafeHtml | undefined => {
    if (!this.server) {
      return undefined;
    }

    var result = this.dataResults.get(this.server.ipaddress);
    if (result) {
      var concatString: string = result.results.join('');
      concatString = this.replaceSubActions(concatString);
      return this.sanitizer.bypassSecurityTrustHtml(concatString);
    }

    return this.sanitizer.bypassSecurityTrustHtml('');
  };

  replaceSubActions = (input: string): string => {
    const groups = input.match(action_regex);

    let output = input;
    if (groups) {
      for (var group of groups) {
        output = output.replace(group, this.generateSubAction(group));
      }
    }

    return output;
  };

  generateSubAction = (regex_group: string): string => {
    const stripped = regex_group.replace('[[', '').replace(']]', '');
    const split = stripped.split(' ');
    const feature_id = this.find(split, 'feature.id');
    const action_id = this.find(split, 'action.id');
    const action_name = this.find(split, 'action.name');
    const action_params = this.find(split, 'action.params');
    const data_id = this.find(split, 'data.id');

    return (
      '<button onclick="MyServerManagerNS.executeSubAction(\'' +
      feature_id +
      "','" +
      action_id +
      "', '" +
      action_name +
      "', '" +
      data_id +
      "','" +
      action_params +
      "','" +
      this.server?.ipaddress +
      '\')">' +
      action_name +
      '</button>'
    );
  };

  find = (split: string[], to_find: string): string | undefined => {
    const found = split.find((s) => s.startsWith(to_find));

    if (found) {
      return found.replace(to_find + '="', '').replace('"', '');
    }
    return undefined;
  };
}
