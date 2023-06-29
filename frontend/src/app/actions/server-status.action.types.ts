import { createAction, props } from '@ngrx/store';
import { Update } from '@ngrx/entity';

export const STATUS_UPDATE_ACTION = '[Server Status] Update';
export const STATUS_INSERT_ACTION = '[Server Status] Insert';
export const STATUS_DELETE_ACTION = '[Server Status] Delete';

export interface ServerStatus {
  ipaddress: string,
  is_running: boolean
}



export const statusUpdate = createAction(STATUS_UPDATE_ACTION, props<{ status: Update<ServerStatus> }>());
export const statusInsert = createAction(STATUS_INSERT_ACTION, props<{ status: ServerStatus }>());
export const statusDelete = createAction(STATUS_DELETE_ACTION, props<{ ipaddress: string }>());
