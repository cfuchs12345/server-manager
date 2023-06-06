import {
  Component,
  Input,
  OnInit,
  OnDestroy,
  OnChanges,
  SimpleChanges,
  ChangeDetectorRef,
} from '@angular/core';
import { time } from 'console';
import { ChartType } from 'ng-apexcharts';
import { Subscription, filter, take } from 'rxjs';
import { MonitoringService } from 'src/app/services/monitoring/monitoring.service';
import { Server } from 'src/app/services/servers/types';
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

  private monitoringDataSubscription: Subscription | undefined;
  private monitorinTimeSeriesSubscription: Subscription | undefined;

  series_id_list: string[] | undefined;

  chartDataList: ChartDataList = new ChartDataList();

  private chartTypes: Map<string, string> = new Map();

  constructor(private monitoringService: MonitoringService, private cdr: ChangeDetectorRef) {}

  ngOnInit(): void {
    this.monitoringDataSubscription = this.monitoringService.data
      .pipe(filter((json) => json !== undefined))
      .subscribe((json) => {
        setTimeout(() => {
          this.update_data_map(json);
        }, 0);
      });
    this.monitorinTimeSeriesSubscription = this.monitoringService.monitoringIds
      .pipe(
        filter((id_list) => id_list !== undefined),
        take(1)
      )
      .subscribe((id_list) => {
        this.series_id_list = id_list;

        setTimeout(() => {
          if (this.server !== undefined && this.series_id_list !== undefined) {
            for (let series_id of this.series_id_list) {
              this.monitoringService.loadMonitoringData(this.server, series_id);
            }
          }
        }, 0);
      });

      if (this.server !== undefined) {
        this.monitoringService.getMonitoringNames(this.server);
      }
  }

  ngOnDestroy(): void {
    if (this.monitoringDataSubscription !== undefined) {
      this.monitoringDataSubscription.unsubscribe();
    }
    if (this.monitorinTimeSeriesSubscription !== undefined) {
      this.monitorinTimeSeriesSubscription.unsubscribe();
    }
  }

  ngOnChanges(changes: SimpleChanges): void {
    for (const propName in changes) {
      if (changes.hasOwnProperty(propName)) {
        switch (propName) {
          case 'server':
            this.series_id_list = undefined;

            setTimeout( () => {
              if (this.server !== undefined) {
                this.monitoringService.getMonitoringNames(this.server);
              }
            },500);
            break;
        }
      }
    }
  }

  update_data_map = (json: any) => {
    if (json !== undefined) {
      if (
        json.hasOwnProperty('series_id') &&
        json.hasOwnProperty('name') &&
        json.hasOwnProperty('series_type') &&
        json.hasOwnProperty('dataset') &&
        json.hasOwnProperty('identifier') &&
        json.hasOwnProperty('value') &&
        json.hasOwnProperty('chart_type')
      ) {

        const dataset = json.dataset as [];
        const series_id = json.series_id as string;
        const chart_name = json.name;
        const identifier_name = json.identifier as string;
        const sub_identifier_name = json.hasOwnProperty('sub_identifier')
          ? (json.sub_identifier as string)
          : '';
        const values_name = json.value as string;
        const series_type = json.series_type as string;
        const chart_type = json.chart_type as ChartType;

        let chartDataListNew = new ChartDataList();

        let series_values: Map<string, number[][]> = new Map();

        let has_sub_identifier = sub_identifier_name.length > 0;

        this.chartTypes.set(series_id, chart_type);



        for (let rowCount = 0; rowCount < dataset.length; rowCount++) {
          const row = dataset[rowCount] as any[];

          const identifier = row[0];
          const sub_identifier = has_sub_identifier ? row[1] : undefined;
          const value = this.getValue(row, has_sub_identifier ? 2 : 1);
          const timestamp = new Date(row[row.length - 1]).getTime();

          let key = has_sub_identifier
            ? sub_identifier + '-' + values_name
            : values_name;

          let series_array = series_values.get(key);
          if (series_array === undefined) {
            series_array = [];
            series_values.set(key, series_array);
          }

          series_array.push([timestamp, value]);
        }

        let chartData = new ChartData(
          series_id,
          chart_name,
          series_type,
          chart_type,
          []
        );
        for (let [key, value] of series_values) {
          chartData.series.push({
            name: key,
            data: value,
          });
        }

        chartDataListNew.list = this.chartDataList.list.slice();
        let existing = chartDataListNew.list.find(
          (cd) => cd.name === chartData.name
        );
        if (existing) {
          let index = chartDataListNew.list.indexOf(existing);
          chartDataListNew.list.splice(index);
        }

        chartDataListNew.list.push(chartData);

        this.chartDataList = chartDataListNew;
      }
    }
  };

  getSeriesIds = (): string[] => {
    const array = [...this.chartDataList.list.map((cd) => cd.name)];

    array.sort((a, b) => a.localeCompare(b));

    return array;
  };

  private getValue = (row: any[], index: number): number => {
    let value = row[index];

    if (value === true) {
      value = 1;
    } else if (value === false) {
      value = 0;
    }
    return parseFloat(value);
  };

  isBarChart = (series_id: string): boolean => {
    return this.chartTypes.get(series_id) === 'bar';
  }

  isLineChart = (series_id: string): boolean => {
    return this.chartTypes.get(series_id) === 'line';
  }
}
