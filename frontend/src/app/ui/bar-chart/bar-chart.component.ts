import {
  Component,
  Input,
  OnChanges,
  ViewChild,
} from '@angular/core';

import { ChartComponent } from 'ng-apexcharts';
import { ChartData, ChartOptions } from 'src/types/ChartData';

@Component({
  selector: 'app-bar-chart',
  templateUrl: './bar-chart.component.html',
  styleUrls: ['./bar-chart.component.scss'],
})
export class BarChartComponent implements OnChanges {
  @Input() series_id: string | undefined;
  @Input() chartData: ChartData | undefined;

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
        type: 'bar',
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
        type: 'datetime',
        tooltip: {
          enabled: false,
        },
      },
      grid: {
        row: {
          colors: ['#f3f3f3', 'transparent'], // takes an array which will be repeated on columns
          opacity: 0.5,
        },
      },
      tooltip: {},
    };
  }

  ngOnChanges(): void {
    if (!this.series_id || !this.chartData) {
      return;
    }

    if (this.chartData) {
      this.chartOptions.series = this.chartData.series;
      if (this.chartOptions.title) {
        this.chartOptions.title.text = this.chartData.name;
      }
      if (
        this.chartOptions.xaxis &&
        (this.chartData.series_type === 'datetime' ||
          this.chartData.series_type === 'category' ||
          this.chartData.series_type === 'numeric')
      ) {
        this.chartOptions.xaxis.type = this.chartData.series_type;

        if (this.chartData.series_type === 'datetime') {
          this.chartOptions.tooltip = {
            x: {
              format: 'dd.MM.yy HH:mm',
            },
          };
        }
      }

      if (this.chart) {
        this.chart.render();
      }
    }
  }
}
