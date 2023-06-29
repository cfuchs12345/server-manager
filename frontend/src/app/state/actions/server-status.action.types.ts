import { createAction, props } from '@ngrx/store';
import { Update } from '@ngrx/entity';
import { Status } from '../../services/servers/types';

export const STATUS_UPDATE_ACTION = '[Server Status] Update';
export const STATUS_INSERT_ACTION = '[Server Status] Insert';
export const STATUS_INSERT_MANY_ACTION = '[Server Status] Insert Many';
export const STATUS_DELETE_ACTION = '[Server Status] Delete';
export const STATUS_INSERT_OR_UPDATE_ACTION = '[Server Status] Insert or Update';


export const updateOne = createAction(STATUS_UPDATE_ACTION, props<{ status: Update<Status> }>());
export const addOne = createAction(STATUS_INSERT_ACTION, props<{ status: Status }>());
export const addMany = createAction(STATUS_INSERT_MANY_ACTION, props<{ status: Status[] }>());
export const removeOne = createAction(STATUS_DELETE_ACTION, props<{ ipaddress: string }>());


export const upsertOne = createAction(STATUS_INSERT_OR_UPDATE_ACTION, props<{ status: Status }>());

