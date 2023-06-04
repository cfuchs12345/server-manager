import { Server } from "../servers/types";

export class QueryMonitoringData {
  constructor(public servers: Server[], public period_from : Number,  public period_from_unit : string,  public period_to: Number,  public period_to_unit: string){}
}
