import { createAction, props } from '@ngrx/store';
import { Update } from '@ngrx/entity';
import { Server } from '../../services/servers/types';

export const SERVER_UPDATE_ACTION = '[Server] Update';
export const SERVER_INSERT_ACTION = '[Server] Insert';
export const SERVER_INSERT_MANY_ACTION = '[Server] Insert Many';
export const SERVER_DELETE_ACTION = '[Server] Delete';
export const SERVER_INSERT_OR_UPDATE_ACTION = '[Server] Insert or Update';


export const updateOne = createAction(SERVER_UPDATE_ACTION, props<{ server: Update<Server> }>());
export const addOne = createAction(SERVER_INSERT_ACTION, props<{ server: Server }>());
export const addMany = createAction(SERVER_INSERT_MANY_ACTION, props<{ servers: Server[] }>());
export const removeOne = createAction(SERVER_DELETE_ACTION, props<{ ipaddress: string }>());


export const upsertOne = createAction(SERVER_INSERT_OR_UPDATE_ACTION, props<{ server: Server }>());

