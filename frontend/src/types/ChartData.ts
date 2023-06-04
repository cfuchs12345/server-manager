import { ApexAxisChartSeries, ApexChart, ApexDataLabels, ApexGrid, ApexStroke, ApexTitleSubtitle, ApexTooltip, ApexXAxis, ChartType } from "ng-apexcharts";

export class ChartData {
  constructor(public series_id: string, public name: string, public series_type: string, public chart_type: ChartType, public series: {name: string, data: number[][]}[]) {}
}

export class ChartDataList {
  constructor(public list: ChartData[] = []) {}
}

export class Series {
  constructor(public name: string, data: number[][] ) {}
}

export type ChartOptions = {
  series: ApexAxisChartSeries;
  chart: ApexChart;
  xaxis: ApexXAxis;
  dataLabels: ApexDataLabels;
  grid: ApexGrid;
  stroke: ApexStroke;
  title: ApexTitleSubtitle,
  tooltip: ApexTooltip
};
