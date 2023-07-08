import {
  Component,
  Input,
  OnInit,
  OnDestroy,
  OnChanges,
  SimpleChanges,
} from '@angular/core';
import { NGXLogger } from 'ngx-logger';
import { MonitoringService } from 'src/app/services/monitoring/monitoring.service';
import {
  SUB_IDENTIFIER,
  SUB_IDENTIFIER2,
  TimeSeriesIds,
  TimeSeriesResponse,
} from 'src/app/services/monitoring/types';
import { Server } from 'src/app/services/servers/types';
import { SubscriptionHandler } from 'src/app/shared/subscriptionHandler';
import { sortByString } from 'src/app/shared/utils';
import { ChartData, ChartDataList } from 'src/types/ChartData';

@Component({
  selector: 'app-monitoring-single-server',
  templateUrl: './monitoring-single-server.component.html',
  styleUrls: ['./monitoring-single-server.component.scss'],
})
export class MonitoringSingleServerComponent
  implements OnInit, OnDestroy, OnChanges
{
  @Input() server: Server | undefined;

  seriesData: TimeSeriesIds | undefined;

  chartDataList: ChartDataList = new ChartDataList();

  chartTypes: Map<string, string> = new Map();

  subscriptionHandler = new SubscriptionHandler(this);

  constructor(
    private monitoringService: MonitoringService,
    private logger: NGXLogger
  ) {}

  ngOnInit(): void {
    this.loadData();
  }

  private loadData() {
    if (this.server) {
      this.subscriptionHandler.subscription = this.monitoringService
        .getMonitoringIds(this.server)
        .subscribe((seriesData) => {
          this.seriesData = seriesData;

          setTimeout(() => {
            if (this.server && this.seriesData) {
              for (const seriesId of this.seriesData.seriesIds) {
                this.subscriptionHandler.subscription = this.monitoringService
                  .loadMonitoringData(this.server, seriesId)
                  .subscribe((data) => {
                    this.updateDataMap(data);
                  });
              }
            }
          }, 0);
        });
    }
  }

  ngOnDestroy(): void {
    this.subscriptionHandler.onDestroy(this.logger);
  }

  ngOnChanges(changes: SimpleChanges): void {
    for (const propName in changes) {
      if (Object.hasOwn(changes, propName)) {
        switch (propName) {
          case 'server':
            this.seriesData = undefined;

            this.loadData();
        }
      }
    }
  }

  updateDataMap = (response: TimeSeriesResponse) => {
    if (response) {
      const dataset = response.data.dataset;

      const series_id = response.meta_data.series_id;
      const chart_name = response.meta_data.name;
      const series_type = response.meta_data.series_type;
      const chart_type = response.meta_data.chart_type;

      const chartDataListNew = new ChartDataList();

      this.chartTypes.set(series_id, chart_type);

      const hasSubIdentifier = this.hasColumn(response, SUB_IDENTIFIER);
      const hasSubIdentifier2 = this.hasColumn(response, SUB_IDENTIFIER2);

      const subIdentifierIndex = hasSubIdentifier ? 1 : -1;
      const subIdentifier2Index = hasSubIdentifier2 ? 2 : -1;
      const valueIndex = hasSubIdentifier ? (hasSubIdentifier2 ? 3 : 2) : 1;
      const timestampIndex = valueIndex + 1;

      const seriesValue = new SeriesValues();

      for (let rowCount = 0; rowCount < dataset.length; rowCount++) {
        // eslint-disable-next-line  @typescript-eslint/no-explicit-any
        const row = dataset[rowCount] as any[];

        const sub_identifier = hasSubIdentifier
          ? row[subIdentifierIndex]
          : undefined;
        const sub_identifier2 = hasSubIdentifier2
          ? row[subIdentifier2Index]
          : undefined;
        const timestamp = new Date(row[timestampIndex]).getTime();
        const value = this.getValue(row, valueIndex);

        const key = this.getKey(
          hasSubIdentifier,
          hasSubIdentifier2,
          sub_identifier,
          sub_identifier2
        );

        seriesValue.add(key, value, timestamp);
      }

      const chartData = new ChartData(
        series_id,
        chart_name,
        series_type,
        chart_type,
        seriesValue.getSeriesData()
      );

      chartDataListNew.list = this.chartDataList.list.slice();
      const existing = chartDataListNew.list.find(
        (cd) => cd.name === chartData.name
      );
      if (existing) {
        const index = chartDataListNew.list.indexOf(existing);
        chartDataListNew.list.splice(index);
      }

      chartDataListNew.list.push(chartData);

      this.chartDataList = chartDataListNew;
    }
  };

  getSeriesIds = (): string[] => {
    const array = [...this.chartDataList.list.map((cd) => cd.name)];
    sortByString(array, (str) => str);

    return array;
  };

  private hasColumn = (response: TimeSeriesResponse, name: string): boolean => {
    return response.data.columns.some((column) => column.name === name);
  };

  // eslint-disable-next-line  @typescript-eslint/no-explicit-any
  private getValue = (row: any[], index: number): number => {
    let value = row[index];

    if (value === true) {
      value = 1;
    } else if (value === false) {
      value = 0;
    }
    return parseFloat(value);
  };

  private getKey = (
    has_sub_identifier: boolean,
    has_sub_identifier2: boolean,
    sub_identifier: string,
    sub_identifier2: string
  ): string => {
    let key = '';

    if (has_sub_identifier) {
      key += sub_identifier;
    }
    if (has_sub_identifier2) {
      key += '-' + sub_identifier2;
    }

    return key;
  };

  getChartData = (seriesId: string): ChartData | undefined => {
    return this.chartDataList.list.find(
      (chartData: ChartData) => chartData.series_id === seriesId
    );
  };
}

class SeriesValues {
  private map: Map<string, number[][]> = new Map();

  add = (key: string, value: number, timestamp: number) => {
    let series_array = this.map.get(key);
    if (!series_array) {
      series_array = [];
      this.map.set(key, series_array);
    }

    series_array.push([timestamp, value]);
  };

  getSeriesData = (): { name: string; data: number[][] }[] => {
    const series_array: { name: string; data: number[][] }[] = [];

    for (const [key, value] of this.map) {
      series_array.push({
        name: key,
        data: value,
      });
    }

    return series_array;
  };
}
