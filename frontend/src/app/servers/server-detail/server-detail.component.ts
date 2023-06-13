import {
  Component,
  Input,
  OnInit,
  OnDestroy,
  OnChanges,
  SimpleChanges,
} from '@angular/core';
import { DomSanitizer, SafeHtml } from '@angular/platform-browser';
import { Subscription, map } from 'rxjs';
import { ServerDataService } from 'src/app/services/servers/server-data.service';
import {
  ConditionCheckResult,
  DataResult,
  Server,
} from 'src/app/services/servers/types';

const action_regex = /\[\[Action.*\]\]/g;

@Component({
  selector: 'app-server-detail',
  templateUrl: './server-detail.component.html',
  styleUrls: ['./server-detail.component.scss'],
})
export class ServerDetailComponent implements OnInit, OnChanges, OnDestroy {
  @Input() server: Server | undefined = undefined;
  @Input() showDetail: boolean = false;
  @Input() turnDetail: boolean = false;

  dataResults: DataResult[] | undefined = undefined;
  dataResultSubscription: Subscription | undefined = undefined;

  innerHtml: SafeHtml;

  constructor(
    private sanitizer: DomSanitizer,
    private serverDataService: ServerDataService
  ) {
    this.innerHtml = sanitizer.bypassSecurityTrustHtml('');
  }

  ngOnInit(): void {
    this.dataResultSubscription = this.serverDataService.dataResults
      .pipe(
        map((data) => {
          return data.filter(
            (d) => this.server && d.ipaddress === this.server.ipaddress
          );
        })
      )
      .subscribe((result) => {
        this.dataResults = result;
        setTimeout(this.formatData, 100);
      });
  }

  ngOnChanges(changes: SimpleChanges): void {
    for (const propName in changes) {
      if (Object.hasOwn(changes, propName)) {
        switch (propName) {
          case 'showDetail': {
            if (this.server && this.showDetail && !this.turnDetail) {
              this.serverDataService.queryData(this.server);
            }
            break;
          }
        }
      }
    }
  }

  ngOnDestroy(): void {
    if (this.dataResultSubscription) {
      this.dataResultSubscription.unsubscribe();
    }
  }

  formatData = () => {
    if (!this.server) {
      return;
    }

    if (this.dataResults) {
      var concatString = this.dataResults.map((d) => d.result).join('');
      concatString = this.replaceSubActions(concatString);
      this.innerHtml = this.sanitizer.bypassSecurityTrustHtml(concatString);
    }
  };

  replaceSubActions = (input: string): string => {
    const groups = input.match(action_regex);

    if (groups === null || groups.length === 0) {
      return input;
    }

    var map: Map<string, GUISubAction> = this.generateSubActions(groups);

    return this.replaceActionsInHTML(input, map);
  };

  replaceActionsInHTML = (
    input: string,
    map: Map<string, GUISubAction>
  ): string => {
    var output = input;
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

    var map: Map<string, GUISubAction> = new Map();
    for (var group of groups) {
      map.set(
        group,
        new GUISubAction(this.server.ipaddress, group, conditionCheckResults)
      );
    }
    return map;
  };

  getConditionCheckResults = (): ConditionCheckResult[] => {
    var list: ConditionCheckResult[] = [];

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
  private conditionMet: boolean = false;
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
    const found = conditionCheckResults.find(
      (res) =>
        res.ipaddress === this.ipaddress &&
        res.feature_id === this.feature_id &&
        res.action_id === this.action_id &&
        res.action_params === this.action_params
    );

    return found !== undefined && found.result;
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
      let inner: string = '';

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

  updateStatusFromResults(subActionCheckResult: ConditionCheckResult[]) {
    for (var c of subActionCheckResult) {
      if (
        c.feature_id === this.feature_id &&
        c.action_id === this.action_id &&
        c.ipaddress === this.ipaddress
      ) {
        this.conditionMet = c.result;
      }
    }
  }
}
