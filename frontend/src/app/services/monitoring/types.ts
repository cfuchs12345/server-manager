import { Server } from "../servers/types";

export class QueryMonitoringData {
  constructor(public servers: Server[], public period_from : Number,  public period_from_unit : string,  public period_to: Number,  public period_to_unit: string){}
}


export class MonitoringSeriesData {
  constructor(public ipaddress: string, public seriesIds: string[]) {}
}

export class MonitoringData {
  private json: any;

  constructor(public ipaddress: string, private data: string) {
    this.json = JSON.parse(data);
  }

  getJson() {
    return this.json;
  }
}
