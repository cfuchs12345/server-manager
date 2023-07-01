import {
  Component,
  Input,
  Output,
  OnInit,
  OnChanges,
  SimpleChanges,
} from '@angular/core';
import { ChartData } from 'src/types/ChartData';

@Component({
  selector: 'app-chart-wrapper',
  templateUrl: './chart-wrapper.component.html',
  styleUrls: ['./chart-wrapper.component.scss'],
})
export class ChartWrapperComponent implements OnInit, OnChanges {
  @Input() series_id: string | undefined;
  @Input() chartData: ChartData | undefined;
  @Input() chartTypes: Map<string, string> = new Map();

  @Output() chartdataToShow: ChartData | undefined;

  selectedGraphs: string[] | undefined;

  ngOnInit(): void {
    this.updateDataToShow();
  }

  ngOnChanges(changes: SimpleChanges): void {
    for (const propName in changes) {
      if (Object.hasOwn(changes, propName)) {
        switch (propName) {
          case 'chartData':
          case 'selectedGraph':
            {
              this.updateDataToShow();
            }
            break;
        }
      }
    }
  }

  onChangeGraph = () => {
    this.updateDataToShow();
  };

  isBarChart = (): boolean => {
    return this.series_id
      ? this.chartTypes.get(this.series_id) === 'bar'
      : false;
  };

  isLineChart = (): boolean => {
    return this.series_id
      ? this.chartTypes.get(this.series_id) === 'line'
      : false;
  };

  hasMultipleGraphs = (): boolean => {
    return this.chartData ? this.chartData.series.length > 1 : false;
  };

  getNames = (): string[] => {
    return this.chartData
      ? this.chartData.series.map((value, index) => {
          return value.name;
        })
      : [];
  };

  updateDataToShow = () => {
    let toShow = undefined;

    if (!this.hasMultipleGraphs()) {
      toShow = this.chartData;
    } else {
      if (this.chartData && this.selectedGraphs) {
        toShow = new ChartData(
          this.chartData.series_id,
          this.chartData.name,
          this.chartData.series_type,
          this.chartData.chart_type,
          this.filterSelected(this.chartData, this.selectedGraphs)
        );
      }
    }
    this.chartdataToShow = toShow;
  };

  filterSelected = (
    chartData: ChartData,
    selectedGraphs: string[]
  ): { name: string; data: number[][] }[] => {
    const series: { name: string; data: number[][] }[] = [];
    for (const selectedGraph of selectedGraphs) {
      const found = chartData.series.find(
        (value) => value.name === selectedGraph
      );
      if (found) {
        series.push(found);
      }
    }
    return series;
  };
}
