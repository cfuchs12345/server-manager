import { Server } from '../servers/types';

export const SUB_IDENTIFIER = 'Sub_Identifier';
export const SUB_IDENTIFIER2 = 'Sub_Identifier2';

export class TimeSeriesQuery {
  constructor(
    public servers: Server[],
    public period_from: number,
    public period_from_unit: string,
    public period_to: number,
    public period_to_unit: string
  ) {}
}

export class TimeSeriesIds {
  constructor(public ipaddress: string, public seriesIds: string[]) {}
}

export class TimeSeriesResponse {
  constructor(
    public meta_data: TimeSeriesResponseMetaData,
    public data: TimeSeriesResponseData
  ) {}
}

export class TimeSeriesResponseMetaData {
  constructor(
    public ipaddress: string,
    public series: string,
    public name: string,
    public series_id: string,
    public series_type: 'category' | 'datetime',
    public chart_type: 'bar' | 'line'
  ) {}
}

export class TimeSeriesResponseData {
  constructor(
    public query: string,
    public columns: TimeSeriesResponseColumnMetaData[],
    public dataset: any[][],
    public count: number
  ) {}
}

export class TimeSeriesResponseColumnMetaData {
  constructor(public name: string, public type: string) {}
}
