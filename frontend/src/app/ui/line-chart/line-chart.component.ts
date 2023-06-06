import {
  Component,
  Input,
  OnInit,
  OnChanges,
  SimpleChanges,
  ViewChild,
} from '@angular/core';

import {
  ChartComponent,
  ApexAxisChartSeries,
  ApexChart,
  ApexXAxis,
  ApexDataLabels,
  ApexTitleSubtitle,
  ApexStroke,
  ApexGrid,
  ApexTooltip,
} from 'ng-apexcharts';
import { ChartDataList, ChartOptions } from 'src/types/ChartData';


@Component({
  selector: 'app-line-chart',
  templateUrl: './line-chart.component.html',
  styleUrls: ['./line-chart.component.scss'],
})
export class LineChartComponent implements OnInit, OnChanges {
  @Input() series_id: string | undefined;
  @Input() chartDataList: ChartDataList | undefined;

  @ViewChild('chart') chart: ChartComponent | undefined;
  public chartOptions: Partial<ChartOptions>;

  constructor() {
    this.chartOptions = {
      series: [
        {
          data: [],
        },
      ],
      chart: {
        width: 500,
        height: 350,
        type: 'line',
        zoom: {
          enabled: false,
        },
      },
      dataLabels: {
        enabled: false,
      },
      stroke: {
        curve: 'straight',
      },
      title: {
        text: '',
        align: 'left',
      },
      xaxis: {
        type: 'datetime'
      },
      yaxis: {
        labels: {
          maxWidth: 250
       }
      },
      grid: {
        row: {
          colors: ['#f3f3f3', 'transparent'], // takes an array which will be repeated on columns
          opacity: 0.5,
        },
      },
      tooltip: {

      }
    };
  }

  ngOnChanges(changes: SimpleChanges): void {
    if (this.series_id === undefined || this.chartDataList === undefined) {
      return;
    }
    const chartData = this.chartDataList.list.find(
      (c) => c.series_id === this.series_id
    );

    if (chartData !== undefined) {
      this.chartOptions.series = chartData.series;

      if (this.chartOptions.title !== undefined) {
        this.chartOptions.title.text = chartData.name;
      }
      if (this.chartOptions.xaxis !== undefined && (chartData.series_type === 'datetime' || chartData.series_type === 'category'|| chartData.series_type === 'numeric')) {
        this.chartOptions.xaxis.type = chartData.series_type;

        if( chartData.series_type === 'datetime' ) {
          this.chartOptions.tooltip = {
            x: {
              format: 'dd.MM.yy HH:mm'
            }
          }
        }
      }
      if( this.chart !== undefined) {
        this.chart.render();
      }
    }
  }

  ngOnInit(): void {}
}
