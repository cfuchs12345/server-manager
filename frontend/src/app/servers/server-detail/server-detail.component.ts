import {
  Component,
  Input,
  OnInit,
  OnChanges,
  OnDestroy,
  inject,
  SimpleChanges,
} from '@angular/core';
import { DomSanitizer, SafeHtml } from '@angular/platform-browser';
import { NGXLogger } from 'ngx-logger';
import { ServerDataService } from 'src/app/services/servers/server-data.service';
import {
  ConditionCheckResult,
  DataResult,
  Server,
} from 'src/app/services/servers/types';
import { SubscriptionHandler } from 'src/app/shared/subscriptionHandler';

const action_regex = /\[\[Action.*\]\]/g;

@Component({
  selector: 'app-server-detail',
  templateUrl: './server-detail.component.html',
  styleUrls: ['./server-detail.component.scss'],
})
export class ServerDetailComponent implements OnInit, OnChanges, OnDestroy {
  private logger = inject(NGXLogger);
  private sanitizer = inject(DomSanitizer);
  private serverDataService = inject(ServerDataService);

  @Input() server: Server | undefined = undefined;
  @Input() showDetail = false;
  @Input() turnDetail = false;

  dataResults: DataResult[] | undefined = undefined;

  innerHtml?: SafeHtml;

  private subscriptionHandler = new SubscriptionHandler(this);

  ngOnInit() {
    if (this.sanitizer) {
      this.innerHtml = this.sanitizer.bypassSecurityTrustHtml('');
    }
  }

  ngOnDestroy(): void {
    this.subscriptionHandler.onDestroy();
  }

  ngOnChanges(changes: SimpleChanges): void {
    for (const propName in changes) {
      if (Object.hasOwn(changes, propName)) {
        switch (propName) {
          case 'showDetail': {
            this.queryData('ngOnChanges');
            break;
          }
        }
      }
    }
  }

  private queryData(source: string) {
    if (this.server && this.showDetail && !this.turnDetail) {
      this.logger.trace('querying data for server ', source, this.server);

      this.subscriptionHandler.subscription = this.serverDataService
        .queryData(this.server)
        .subscribe((result) => {
          this.dataResults = result;
          setTimeout(this.formatData, 0);
        });
    }
  }

  formatData = () => {
    if (!this.server) {
      return;
    }

    if (this.dataResults) {
      let concatString = this.dataResults.map((d) => d.result).join('');
      concatString = this.replaceSubActions(concatString);
      this.innerHtml = this.sanitizer.bypassSecurityTrustHtml(concatString);
    }
  };

  replaceSubActions = (input: string): string => {
    const groups = input.match(action_regex);

    if (groups === null || groups.length === 0) {
      return input;
    }

    const map: Map<string, GUISubAction> = this.generateSubActions(groups);

    return this.replaceActionsInHTML(input, map);
  };

  replaceActionsInHTML = (
    input: string,
    map: Map<string, GUISubAction>
  ): string => {
    let output = input;
    for (const entry of map.entries()) {
      const to_replace = entry[0];
      const subAction = entry[1];

      output = output.replace(to_replace, subAction.generateUIElement());
    }

    return output;
  };

  generateSubActions = (
    groups: RegExpMatchArray
  ): Map<string, GUISubAction> => {
    if (!this.server) {
      return new Map();
    }
    const conditionCheckResults = this.getConditionCheckResults();

    const map: Map<string, GUISubAction> = new Map();
    for (const group of groups) {
      map.set(
        group,
        new GUISubAction(this.server.ipaddress, group, conditionCheckResults)
      );
    }
    return map;
  };

  getConditionCheckResults = (): ConditionCheckResult[] => {
    const list: ConditionCheckResult[] = [];

    if (!this.dataResults) {
      return list;
    }
    for (const dataResult of this.dataResults) {
      list.push(...dataResult.check_results);
    }
    return list;
  };
}

class GUISubAction {
  private conditionMet = false;
  private feature_id: string | undefined;
  private action_id: string | undefined;
  private action_name: string | undefined;
  private action_params: string | undefined;
  private action_image: string | undefined;
  private data_id: string | undefined;

  constructor(
    private ipaddress: string,
    regexGroup: string,
    conditionCheckResults: ConditionCheckResult[]
  ) {
    const stripped = regexGroup.replace('[[', '').replace(']]', '');
    const split = stripped.split(' ');
    this.feature_id = this.find(split, 'feature.id');
    this.action_id = this.find(split, 'action.id');
    this.action_name = this.find(split, 'action.name');
    this.action_params = this.find(split, 'action.params');
    this.action_image = this.find(split, 'action.image');
    this.data_id = this.find(split, 'data.id');

    this.conditionMet = this.checkCondition(conditionCheckResults);
  }

  checkCondition = (conditionCheckResults: ConditionCheckResult[]): boolean => {
    const foundResultByIp = conditionCheckResults.find(
      (res) => res.ipaddress === this.ipaddress && res.data_id === this.data_id
    );

    if (foundResultByIp !== undefined) {
      const foundSubResult = foundResultByIp.subresults.find(
        (res) =>
          res.feature_id === this.feature_id &&
          res.action_id === this.action_id &&
          res.action_params === this.action_params
      );

      return foundSubResult !== undefined && foundSubResult.result;
    }
    return false;
  };

  find = (split: string[], to_find: string): string | undefined => {
    const found = split.find((s) => s.startsWith(to_find));

    if (found) {
      return found.replace(to_find + '="', '').replace('"', '');
    }
    return undefined;
  };

  generateUIElement = (): string => {
    if (this.conditionMet) {
      let inner = '';

      if (this.action_image && this.action_image !== '') {
        return (
          '<input type="image" src="' +
          this.action_image +
          '" alt="' +
          this.action_name +
          '" onclick="MyServerManagerNS.executeSubAction(\'' +
          this.feature_id +
          "','" +
          this.action_id +
          "', '" +
          this.action_name +
          "', '" +
          this.data_id +
          "','" +
          this.action_params +
          "','" +
          this.ipaddress +
          '\')"></input>'
        );
      } else if (this.action_name) {
        inner = this.action_name;

        return (
          '<button onclick="MyServerManagerNS.executeSubAction(\'' +
          this.feature_id +
          "','" +
          this.action_id +
          "', '" +
          this.action_name +
          "', '" +
          this.data_id +
          "','" +
          this.action_params +
          "','" +
          this.ipaddress +
          '\')">' +
          inner +
          '</button>'
        );
      }
    }
    return '';
  };
}
